use pgvector::Vector;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::chunks::Chunk;

impl Chunk {
    pub async fn semantic_search(
        pool: &PgPool,
        collection_id: Uuid,
        embedding: Vec<f32>,
        limit: i32,
    ) -> sqlx::Result<Vec<Self>> {
        let embedding = Vector::from(embedding);

        sqlx::query_as(
            r#"
            SELECT chunks.id, document_id, text, language::text
            FROM chunks
            JOIN documents ON documents.id = document_id
            WHERE documents.collection_id = $2
            ORDER BY $1 <=> embedding
            LIMIT $3
            "#,
        )
        .bind(embedding)
        .bind(collection_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}
