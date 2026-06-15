use carmen_db::documents::{Document, DocumentIndexing};
use carmen_db::types::Status;
use carmen_s3::Storage;
use log::{error, info};
use sqlx::PgPool;
use tempfile::TempDir;
use tokio::sync::oneshot::{self, Receiver, Sender};
use uuid::Uuid;

pub struct Task {
    id: Uuid,
    pool: PgPool,
    storage: Storage,
    cancel_rx: Receiver<()>,
}

impl Task {
    pub fn new(id: Uuid, pool: PgPool, storage: Storage) -> (Self, Sender<()>) {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let task = Self {
            id,
            pool,
            storage,
            cancel_rx,
        };

        (task, cancel_tx)
    }

    pub async fn start(self) -> anyhow::Result<()> {
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

    async fn run(&self) -> anyhow::Result<()> {
        let indexing = match DocumentIndexing::claim(&self.pool, self.id).await? {
            Some(claimed) => claimed,
            None => return Ok(()),
        };

        let document = Document::get(&self.pool, indexing.document_id).await?;

        info!(
            "Started indexing {} of document {}",
            indexing.id, document.id,
        );

        let tempdir = TempDir::with_prefix("carmen_indexer-")?;

        // TODO: index document

        Ok(())
    }
}
