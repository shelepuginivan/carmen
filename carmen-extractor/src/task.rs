use carmen_db::collections::{Collection, CollectionExtraction};
use log::{error, info, warn};
use sqlx::PgPool;
use tempfile::TempDir;
use tokio::sync::oneshot::{self, Receiver, Sender};
use uuid::Uuid;

use crate::extractors::{EXTRACTORS, Extractor};

pub struct Task {
    id: Uuid,
    pool: PgPool,
    cancel_rx: Receiver<()>,
}

impl Task {
    pub fn new(pool: PgPool, id: Uuid) -> (Self, Sender<()>) {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let task = Self {
            id,
            pool,
            cancel_rx,
        };

        (task, cancel_tx)
    }

    pub async fn start(&self) -> anyhow::Result<()> {
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

        let extracted = match EXTRACTORS.iter().find(|ex| ex.can_extract(&collection)) {
            Some(ex) => match ex.extract(&collection, tempdir.path()).await {
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

        for doc in extracted {
            println!("{} :: {}", doc.file_path.display(), doc.canonical_path)
        }

        Ok(())
    }
}
