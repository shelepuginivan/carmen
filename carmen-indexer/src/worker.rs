use anyhow::Context;
use carmen_db::documents::DocumentIndexing;
use carmen_db::{chunks::Chunk, types::Status};
use carmen_s3::Storage;
use fastembed::{InitOptions, TextEmbedding};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use log::{error, info};
use sqlx::PgPool;
use text_splitter::{Characters, MarkdownSplitter};
use tokio::select;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::config::Config;

struct Task {
    id: Uuid,
}

struct WorkerActor {
    stop: watch::Receiver<bool>,
    tasks: mpsc::Receiver<Task>,

    pool: PgPool,
    storage: Storage,
    embedder: TextEmbedding,
    splitter: MarkdownSplitter<Characters>,
    detector: LanguageDetector,
    batch_size: Option<usize>,
}

pub struct WorkerHandle {
    stop: watch::Sender<bool>,
    tasks: mpsc::Sender<Task>,
    handle: JoinHandle<()>,
}

impl WorkerHandle {
    pub fn new(config: &Config, pool: PgPool) -> anyhow::Result<Self> {
        let (stop_tx, stop_rx) = watch::channel(false);
        let (tasks_tx, tasks_rx) = mpsc::channel(16);

        let mut actor = WorkerActor::new(config, pool, tasks_rx, stop_rx)?;
        let handle = tokio::spawn(async move { actor.run().await });

        Ok(Self {
            stop: stop_tx,
            tasks: tasks_tx,
            handle,
        })
    }

    pub async fn push_task(&self, id: Uuid) {
        let task = Task { id };
        let _ = self.tasks.send(task).await;
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.stop.send(true).context("failed to stop actor")?;
        info!("Waiting for the ongoing indexing to complete...");
        self.handle.await.context("failed to join actor handle")?;

        Ok(())
    }
}

impl WorkerActor {
    pub fn new(
        config: &Config,
        pool: PgPool,
        tasks_rx: mpsc::Receiver<Task>,
        cancel_rx: watch::Receiver<bool>,
    ) -> anyhow::Result<Self> {
        let mut options = InitOptions::new(config.embedding_model.clone());

        if let Some(intra_threads) = config.embedding_threads {
            options = options.with_intra_threads(intra_threads);
        }

        let embedder = TextEmbedding::try_new(options)?;
        let storage = Storage::new_from_env()?;
        let splitter = MarkdownSplitter::new(config.max_chunk_size);
        let detector = LanguageDetectorBuilder::from_languages(&config.languages).build();

        Ok(Self {
            tasks: tasks_rx,
            stop: cancel_rx,
            pool,
            storage,
            embedder,
            splitter,
            detector,
            batch_size: config.embedding_batch_size,
        })
    }

    pub async fn run(&mut self) {
        loop {
            select! {
                _ = self.stop.changed() => {
                    if *self.stop.borrow() {
                        break;
                    }
                }
                Some(task) = self.tasks.recv() => {
                    let _ = self.process_task(task).await;
                }
            }
        }
    }

    async fn process_task(&mut self, task: Task) -> anyhow::Result<()> {
        let indexing = match DocumentIndexing::claim(&self.pool, task.id).await? {
            Some(i) => i,
            None => return Ok(()),
        };

        let status = match self.do_indexing(&indexing).await {
            Ok(_) => {
                info!("Indexing {} completed successfully", indexing.id);
                Status::Completed
            }
            Err(err) => {
                error!("Indexing {} failed: {err}", indexing.id);
                Status::Failed
            }
        };

        DocumentIndexing::update_status(&self.pool, indexing.id, status).await?;

        Ok(())
    }

    async fn do_indexing(&mut self, indexing: &DocumentIndexing) -> anyhow::Result<()> {
        info!("Started indexing {}...", indexing.id);

        let document_str = self
            .storage
            .get_exported_document_as_string(indexing.document_id)
            .await?;

        let fragments: Vec<&str> = self.splitter.chunks(&document_str).collect();
        let embeddings = self.embedder.embed(&fragments, self.batch_size)?;

        for (embedding, fragment) in embeddings.into_iter().zip(fragments) {
            let lang = self
                .detector
                .detect_language_of(fragment)
                .map(|lang| lang.to_string())
                .unwrap_or_else(|| "simple".to_string());

            Chunk::insert(&self.pool, indexing.document_id, fragment, &lang, embedding).await?;
        }

        Ok(())
    }
}
