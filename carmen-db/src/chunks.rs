use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::documents::Document;

#[derive(sqlx::FromRow)]
pub struct Chunk {
    pub id: Uuid,
    pub document_id: Uuid,
    pub text: String,
    pub language: String,
}

impl Chunk {
    pub async fn insert(
        pool: &PgPool,
        document_id: Uuid,
        text: &str,
        language: &str,
        embedding: Vec<f32>,
    ) -> sqlx::Result<Self> {
        Document::assert_exists(pool, document_id).await?;

        let embedding = pgvector::Vector::from(embedding);

        sqlx::query_as(
            r#"
            INSERT INTO chunks (document_id, text, language, embedding)
            VALUES ($1, $2, $3::regconfig, $4)
            RETURNING id, document_id, text, language::text
            "#,
        )
        .bind(document_id)
        .bind(text)
        .bind(language)
        .bind(embedding)
        .fetch_one(pool)
        .await
    }

    pub async fn delete_by_document_id(pool: &PgPool, document_id: Uuid) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM chunks WHERE document_id = $1")
            .bind(document_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
