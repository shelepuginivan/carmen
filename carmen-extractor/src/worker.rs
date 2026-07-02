use anyhow::{Context, bail};
use carmen_db::collections::Collection;
use carmen_db::documents::Document;
use carmen_db::extractions::{Extraction, ExtractionStatus};
use carmen_s3::Storage;
use log::{error, info};
use sqlx::PgPool;
use tempfile::TempDir;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

use crate::documents::{DocumentDiff, DocumentUpdater};
use crate::extractors::{EXTRACTORS, Extractor};

struct WorkerActor {
    stop: watch::Receiver<bool>,
    tasks: mpsc::Receiver<Extraction>,

    pool: PgPool,
    storage: Storage,
}

pub struct WorkerHandle {
    stop: watch::Sender<bool>,
    tasks: mpsc::Sender<Extraction>,
    handle: JoinHandle<()>,
}

impl WorkerHandle {
    pub fn new(pool: PgPool) -> anyhow::Result<Self> {
        let (stop_tx, stop_rx) = watch::channel(false);
        let (tasks_tx, tasks_rx) = mpsc::channel(16);

        let mut actor = WorkerActor::new(pool, tasks_rx, stop_rx)?;
        let handle = tokio::spawn(async move { actor.run().await });

        Ok(Self {
            stop: stop_tx,
            tasks: tasks_tx,
            handle,
        })
    }

    pub async fn push_extraction(&self, task: Extraction) {
        let _ = self.tasks.send(task).await;
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.stop.send(true).context("failed to stop actor")?;
        info!("Waiting for the ongoing extraction to complete...");
        self.handle.await.context("failed to join actor handle")?;

        Ok(())
    }
}

impl WorkerActor {
    pub fn new(
        pool: PgPool,
        tasks_rx: mpsc::Receiver<Extraction>,
        cancel_rx: watch::Receiver<bool>,
    ) -> anyhow::Result<Self> {
        let storage = Storage::new_from_env()?;

        Ok(Self {
            tasks: tasks_rx,
            stop: cancel_rx,
            pool,
            storage,
        })
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                _ = self.stop.changed() => {
                    if *self.stop.borrow() {
                        break;
                    }
                }
                Some(task) = self.tasks.recv() => {
                    let _ = self.process_extraction(task).await;
                }
            }
        }
    }

    async fn process_extraction(&mut self, extraction: Extraction) -> anyhow::Result<()> {
        let status = match self.do_extraction(&extraction).await {
            Ok(_) => {
                info!("Extraction {} completed successfully", extraction.id);
                ExtractionStatus::Completed
            }
            Err(err) => {
                error!("Extraction {} failed: {err}", extraction.id);
                ExtractionStatus::Failed
            }
        };

        Extraction::update_status(&self.pool, extraction.id, status).await?;

        Ok(())
    }

    async fn do_extraction(&mut self, extraction: &Extraction) -> anyhow::Result<()> {
        let source_type = match extraction.source_type.parse() {
            Ok(et) => et,
            Err(_) => bail!(
                "Unknown source type '{}' (extraction {})",
                extraction.source_type,
                extraction.id
            ),
        };

        let extractor = EXTRACTORS
            .get(&source_type)
            .expect("known source type should have matching extractor");

        info!("Started extraction {}...", extraction.id);

        let collection = Collection::get(&self.pool, extraction.collection_id).await?;
        let tempdir = TempDir::with_prefix("carmen_extractor-")?;
        let extracted = extractor.extract(extraction, tempdir.path()).await?;
        let documents = Document::get_for_collection(&self.pool, collection.id).await?;
        let diff = DocumentDiff::compute(documents, extracted).await?;

        DocumentUpdater::new(&self.pool, &self.storage)
            .update(extraction, &diff)
            .await?;

        info!(
            "Collection '{}': added {}, updated {}, removed {} documents",
            collection.name,
            diff.added.len(),
            diff.updated.len(),
            diff.removed.len()
        );

        Ok(())
    }
}
