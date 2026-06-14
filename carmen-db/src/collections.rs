use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

use super::types::Status;

pub const COLLECTION_EXTRACTION_CHAN: &str = "carmen_collection_extraction";

#[derive(sqlx::FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub source: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct CollectionExtraction {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

impl Collection {
    pub async fn insert(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
        url: Option<&str>,
        source: Option<&str>,
    ) -> sqlx::Result<Self> {
        sqlx::query_as(
            "INSERT INTO collections (name, description, url, source) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(name)
        .bind(description)
        .bind(url)
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

impl CollectionExtraction {
    pub async fn claim(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        let mut tx = pool.begin().await?;

        let extraction = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM collection_extractions
            WHERE id = $1 AND status = $2
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(id)
        .bind(Status::Pending)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(claimed) = extraction {
            sqlx::query("UPDATE collection_extractions SET status = $1 WHERE id = $2")
                .bind(Status::InProgress)
                .bind(id)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            Ok(Some(claimed))
        } else {
            tx.rollback().await?;
            Ok(None)
        }
    }

    pub async fn update_status(pool: &PgPool, id: Uuid, new_status: Status) -> sqlx::Result<()> {
        sqlx::query("UPDATE collection_extractions SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }
}
