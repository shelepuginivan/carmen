use carmen_db::collections::{CollectionTask, CollectionTaskMeta};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct CollectionTaskMetaOut {
    pub id: String,
    pub collection_id: String,
}

impl From<CollectionTaskMeta> for CollectionTaskMetaOut {
    fn from(value: CollectionTaskMeta) -> Self {
        Self {
            id: value.id.to_string(),
            collection_id: value.collection_id.to_string(),
        }
    }
}

// FIXME: create proper error types
pub async fn retry_failed_tasks(db: &PgPool) -> anyhow::Result<Vec<CollectionTaskMetaOut>> {
    Ok(CollectionTask::retry_failed(db)
        .await?
        .into_iter()
        .map(CollectionTaskMetaOut::from)
        .collect())
}
