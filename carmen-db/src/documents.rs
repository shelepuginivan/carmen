use sqlx::PgPool;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub canonical_path: String,
    pub checksum: [u8; 32],
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
            .await
            .map(|_| ())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<Self> {
        sqlx::query_as("DELETE FROM documents WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}
