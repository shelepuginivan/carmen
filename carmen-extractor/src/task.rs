use carmen_db::collections::{Collection, CollectionExtraction};
use log::info;
use sqlx::PgPool;
use tokio::sync::oneshot::{self, Receiver, Sender};
use uuid::Uuid;

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

        Ok(())
    }
}
