use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::Type)]
#[sqlx(type_name = "indexing_status", rename_all = "snake_case")]
pub enum IndexingStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(sqlx::FromRow)]
pub struct Indexing {
    pub id: Uuid,
    pub document_id: Uuid,
    pub status: IndexingStatus,
    pub created_at: DateTime<Utc>,
}

impl Indexing {
    pub async fn claim(pool: &PgPool) -> sqlx::Result<Option<Self>> {
        let mut tx = pool.begin().await?;

        let extraction = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM indexing
            WHERE status = $1
            ORDER BY created_at
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(IndexingStatus::Pending)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(claimed) = extraction {
            sqlx::query("UPDATE indexing SET status = $1 WHERE id = $2")
                .bind(IndexingStatus::InProgress)
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
        new_status: IndexingStatus,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE indexing SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_for_document(pool: &PgPool, document_id: Uuid) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM indexing WHERE document_id = $1 ORDER BY created_at DESC")
            .bind(document_id)
            .fetch_all(pool)
            .await
    }

    pub async fn schedule(pool: &PgPool, document_id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("INSERT INTO indexing (document_id) VALUES ($1) RETURNING *")
            .bind(document_id)
            .fetch_one(pool)
            .await
    }
}
