use std::sync::{Arc, Mutex};

use carmen_db::chunks::Chunk;
use carmen_db::documents::{Document, DocumentIndexing};
use carmen_db::types::Status;
use carmen_s3::Storage;
use log::{error, info};
use sqlx::PgPool;
use tokio::sync::oneshot::{self, Receiver, Sender};
use uuid::Uuid;

use crate::indexer::Indexer;

pub struct Task {
    id: Uuid,
    pool: PgPool,
    storage: Storage,
    indexer: Arc<Mutex<Indexer>>,
    cancel_rx: Receiver<()>,
}

impl Task {
    pub fn new(
        id: Uuid,
        pool: PgPool,
        storage: Storage,
        indexer: Arc<Mutex<Indexer>>,
    ) -> (Self, Sender<()>) {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let task = Self {
            id,
            pool,
            storage,
            indexer,
            cancel_rx,
        };

        (task, cancel_tx)
    }

    pub async fn start(mut self) -> anyhow::Result<()> {
        let status = match self.run().await {
            Ok(_) => {
                info!("Indexing {} completed successfully", self.id);
                Status::Completed
            }
            Err(err) => {
                error!("Indexing {} failed: {err}", self.id);
                Status::Failed
            }
        };

        DocumentIndexing::update_status(&self.pool, self.id, status).await?;
        Ok(())
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        let indexing = match DocumentIndexing::claim(&self.pool, self.id).await? {
            Some(claimed) => claimed,
            None => return Ok(()),
        };

        let document = Document::get(&self.pool, indexing.document_id).await?;

        info!(
            "Started indexing {} of document {}",
            indexing.id, document.id,
        );

        let document_str = self
            .storage
            .get_exported_document_as_string(document.id)
            .await?;

        let chunks = self.indexer.lock().unwrap().embed_document(&document_str)?;

        for chunk in chunks {
            Chunk::insert(
                &self.pool,
                document.id,
                chunk.text,
                "simple", // TODO: detect language
                chunk.embedding,
            )
            .await?;
        }

        Ok(())
    }
}
