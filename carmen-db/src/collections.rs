use sqlx::types::Uuid;
use sqlx::{FromRow, PgExecutor};

#[derive(sqlx::Type)]
#[sqlx(type_name = "collection_task_status", rename_all = "lowercase")]
pub enum CollectionTaskStatus {
    Pending,
    Extracting,
    Indexing,
    Completed,
    Failed,
}

#[derive(sqlx::FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub source: String,
}

#[derive(sqlx::FromRow)]
pub struct CollectionTask {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: CollectionTaskStatus,
}

#[derive(sqlx::FromRow)]
pub struct CollectionTaskMeta {
    pub id: Uuid,
    pub collection_id: Uuid,
}

impl CollectionTask {
    pub async fn retry_failed(
        executor: impl PgExecutor<'_>,
    ) -> sqlx::Result<Vec<CollectionTaskMeta>> {
        sqlx::query(
            "UPDATE collection_tasks SET status = $1 WHERE status = $2 RETURNING id, collection_id",
        )
        .bind(CollectionTaskStatus::Pending)
        .bind(CollectionTaskStatus::Failed)
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|r| CollectionTaskMeta::from_row(&r))
        .collect()
    }
}
