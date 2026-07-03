use std::time::Duration;

use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

pub const EXTRACTION_DELAY: Duration = Duration::from_secs(10);

#[derive(PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "extraction_status", rename_all = "snake_case")]
pub enum ExtractionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "extraction_type", rename_all = "snake_case")]
pub enum ExtractionType {
    Merge,
    Override,
}

#[derive(sqlx::FromRow)]
pub struct Extraction {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: ExtractionStatus,
    pub source: String,
    pub source_type: String,
    pub extraction_type: ExtractionType,
    pub parameters: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl Extraction {
    pub async fn get_for_collection(pool: &PgPool, collection_id: Uuid) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as(
            r#"
            SELECT * FROM extractions
            WHERE collection_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(collection_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("SELECT * FROM extractions WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn claim(pool: &PgPool) -> sqlx::Result<Option<Self>> {
        let mut tx = pool.begin().await?;

        let extraction = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM extractions
            WHERE status = $1 AND created_at < $2
            ORDER BY created_at
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(ExtractionStatus::Pending)
        .bind(Utc::now() - EXTRACTION_DELAY)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(claimed) = extraction {
            sqlx::query("UPDATE extractions SET status = $1 WHERE id = $2")
                .bind(ExtractionStatus::InProgress)
                .bind(claimed.id)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            Ok(Some(claimed))
        } else {
            tx.rollback().await?;
            Ok(None)
        }
    }

    pub async fn update_status(
        pool: &PgPool,
        id: Uuid,
        new_status: ExtractionStatus,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE extractions SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn schedule(
        pool: &PgPool,
        collection_id: Uuid,
        source: &str,
        source_type: &str,
        extraction_type: ExtractionType,
        parameters: &serde_json::Value,
    ) -> sqlx::Result<Self> {
        sqlx::query_as(
            r#"
            INSERT INTO extractions
            (collection_id, source, source_type, extraction_type, parameters)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(collection_id)
        .bind(source)
        .bind(source_type)
        .bind(extraction_type)
        .bind(parameters)
        .fetch_one(pool)
        .await
    }

    pub async fn cancel(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
        let mut tx = pool.begin().await?;

        let extraction = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM extractions
            WHERE id = $1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;

        if extraction.status != ExtractionStatus::Pending {
            tx.rollback().await?;
            return Ok(false);
        }

        if extraction.created_at + EXTRACTION_DELAY < Utc::now() {
            tx.rollback().await?;
            return Ok(false);
        }

        sqlx::query("UPDATE extractions SET status = $1 WHERE id = $2")
            .bind(ExtractionStatus::Cancelled)
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(true)
    }
}
