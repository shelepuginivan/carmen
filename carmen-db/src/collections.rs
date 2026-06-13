use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

pub const COLLECTION_EXTRACTION_CHAN: &str = "carmen_collection_extraction";

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
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
        source: Option<&str>,
    ) -> sqlx::Result<Self> {
        sqlx::query_as(
            "INSERT INTO collections (name, description, source) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(description)
        .bind(source)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("SELECT * FROM collections WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_extractions(
        pool: &PgPool,
        id: Uuid,
    ) -> sqlx::Result<Vec<CollectionExtraction>> {
        sqlx::query_as(
            "SELECT * FROM collection_extractions WHERE collection_id = $1 ORDER BY created_at DESC",
        )
        .bind(id)
        .fetch_all(pool)
        .await
    }

    pub async fn schedule_extraction(
        pool: &PgPool,
        id: Uuid,
    ) -> sqlx::Result<CollectionExtraction> {
        let extraction: CollectionExtraction = sqlx::query_as(
            "INSERT INTO collection_extractions (collection_id) VALUES ($1) RETURNING *",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        sqlx::query("SELECT pg_notify($1, $2)")
            .bind(COLLECTION_EXTRACTION_CHAN)
            .bind(extraction.id.to_string())
            .execute(pool)
            .await?;

        Ok(extraction)
    }
}
