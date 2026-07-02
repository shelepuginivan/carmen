use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub canonical_path: String,
    pub checksum: [u8; 32],
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "document_indexing_status", rename_all = "snake_case")]
pub enum DocumentIndexingStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(sqlx::FromRow)]
pub struct DocumentIndexing {
    pub id: Uuid,
    pub document_id: Uuid,
    pub status: DocumentIndexingStatus,
    pub created_at: DateTime<Utc>,
}

impl Document {
    pub async fn insert(
        pool: &PgPool,
        collection_id: Uuid,
        canonical_path: &str,
        checksum: [u8; 32],
    ) -> sqlx::Result<Self> {
        sqlx::query_as(
            r#"
            INSERT INTO documents (collection_id, canonical_path, checksum)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(collection_id)
        .bind(canonical_path)
        .bind(checksum)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("SELECT * FROM documents WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_for_collection(pool: &PgPool, collection_id: Uuid) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as("SELECT * FROM documents WHERE collection_id = $1")
            .bind(collection_id)
            .fetch_all(pool)
            .await
    }

    pub async fn update_checksum(pool: &PgPool, id: Uuid, checksum: [u8; 32]) -> sqlx::Result<()> {
        sqlx::query("UPDATE documents SET checksum = $1 WHERE id = $2")
            .bind(checksum)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("DELETE FROM documents WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_indexing(pool: &PgPool, id: Uuid) -> sqlx::Result<Vec<DocumentIndexing>> {
        sqlx::query_as(
            "SELECT * FROM document_indexing WHERE document_id = $1 ORDER BY created_at DESC",
        )
        .bind(id)
        .fetch_all(pool)
        .await
    }

    pub async fn schedule_indexing(pool: &PgPool, id: Uuid) -> sqlx::Result<DocumentIndexing> {
        sqlx::query_as("INSERT INTO document_indexing (document_id) VALUES ($1) RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}

impl DocumentIndexing {
    pub async fn claim(pool: &PgPool) -> sqlx::Result<Option<Self>> {
        let mut tx = pool.begin().await?;

        let extraction = sqlx::query_as::<_, Self>(
            r#"
            SELECT * FROM document_indexing
            WHERE status = $1
            ORDER BY created_at
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(DocumentIndexingStatus::Pending)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(claimed) = extraction {
            sqlx::query("UPDATE document_indexing SET status = $1 WHERE id = $2")
                .bind(DocumentIndexingStatus::InProgress)
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
        new_status: DocumentIndexingStatus,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE document_indexing SET status = $1 WHERE id = $2")
            .bind(new_status)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
