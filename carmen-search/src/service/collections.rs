use carmen_db::collections::{CollectionTask, CollectionTaskMeta};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use super::error::Result;

#[derive(Serialize, ToSchema)]
#[schema(title = "CollectionTaskMeta")]
pub struct CollectionTaskMetaOut {
    pub id: Uuid,
    pub collection_id: Uuid,
}

impl From<CollectionTaskMeta> for CollectionTaskMetaOut {
    fn from(value: CollectionTaskMeta) -> Self {
        Self {
            id: value.id,
            collection_id: value.collection_id,
        }
    }
}

pub async fn retry_failed_tasks(db: &PgPool) -> Result<Vec<CollectionTaskMetaOut>> {
    Ok(CollectionTask::retry_failed(db)
        .await?
        .into_iter()
        .map(CollectionTaskMetaOut::from)
        .collect())
}
