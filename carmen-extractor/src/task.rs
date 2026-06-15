use carmen_db::collections::{Collection, CollectionExtraction};
use carmen_db::documents::Document;
use carmen_db::types::Status;
use log::{error, info, warn};
use s3::Bucket;
use sqlx::PgPool;
use tempfile::TempDir;
use tokio::sync::oneshot::{self, Receiver, Sender};
use uuid::Uuid;

use crate::documents::{DocumentDiff, DocumentUpdater};
use crate::extractors::{EXTRACTORS, Extractor};

pub struct Task {
    id: Uuid,
    pool: PgPool,
    bucket: Box<Bucket>,
    cancel_rx: Receiver<()>,
}

impl Task {
    pub fn new(id: Uuid, pool: PgPool, bucket: Box<Bucket>) -> (Self, Sender<()>) {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let task = Self {
            id,
            pool,
            bucket,
            cancel_rx,
        };

        (task, cancel_tx)
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let status = match self.run().await {
            Ok(_) => {
                info!("Extraction {} completed successfully", self.id);
                Status::Completed
            }
            Err(err) => {
                error!("Extraction {} failed: {err}", self.id);
                Status::Failed
            }
        };

        CollectionExtraction::update_status(&self.pool, self.id, status).await?;
        Ok(())
    }

    async fn run(&self) -> anyhow::Result<()> {
        let extraction = match CollectionExtraction::claim(&self.pool, self.id).await? {
            Some(claimed) => claimed,
            None => return Ok(()),
        };

        let collection = Collection::get(&self.pool, extraction.collection_id).await?;

        info!(
            "Started extraction {} of collection '{}' ({})",
            extraction.id, collection.name, collection.id
        );

        let tempdir = TempDir::with_prefix("carmen_extractor-")?;

        let extracted = match EXTRACTORS.iter().find(|ex| ex.can_extract(&extraction)) {
            Some(ex) => match ex.extract(&extraction, tempdir.path()).await {
                Ok(ex) => ex,
                Err(err) => {
                    error!(
                        "Error occurred while extracting collection '{}': {}",
                        collection.name, err
                    );
                    return Err(err);
                }
            },
            None => {
                warn!(
                    "Could not find extractor for collection '{}'",
                    collection.name
                );
                return Ok(());
            }
        };

        let documents = Document::get_for_collection(&self.pool, collection.id).await?;

        let diff = DocumentDiff::compute(documents, extracted).await?;

        DocumentUpdater::new(&self.pool, &self.bucket)
            .update(&extraction, &diff)
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
