use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, PgExecutor};

#[derive(sqlx::Type)]
#[sqlx(type_name = "collection_extraction_status", rename_all = "lowercase")]
pub enum CollectionExtractionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(sqlx::FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub source: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct CollectionExtraction {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: CollectionExtractionStatus,
    pub created_at: DateTime<Utc>,
}

impl Collection {
    pub async fn insert(
        executor: impl PgExecutor<'_>,
        name: &str,
        description: Option<&str>,
        source: Option<&str>,
    ) -> sqlx::Result<Self> {
        sqlx::query(
            "INSERT INTO collections (name, description, source) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(description)
        .bind(source)
        .fetch_one(executor)
        .await
        .and_then(|r| Collection::from_row(&r))
    }

    pub async fn get(executor: impl PgExecutor<'_>, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query("SELECT * FROM collections WHERE id = $1")
            .bind(id)
            .fetch_one(executor)
            .await
            .and_then(|r| Collection::from_row(&r))
    }

    pub async fn get_extractions(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> sqlx::Result<Vec<CollectionExtraction>> {
        sqlx::query(
            "SELECT * FROM collection_extractions WHERE collection_id = $1 ORDER BY created_at DESC",
        )
        .bind(id)
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|r| CollectionExtraction::from_row(&r))
        .collect()
    }

    pub async fn schedule_extraction(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> sqlx::Result<CollectionExtraction> {
        sqlx::query("INSERT INTO collection_extractions (collection_id) VALUES ($1) RETURNING *")
            .bind(id)
            .fetch_one(executor)
            .await
            .and_then(|r| CollectionExtraction::from_row(&r))
    }
}
