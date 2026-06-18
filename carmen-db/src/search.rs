use pgvector::Vector;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::chunks::Chunk;

impl Chunk {
    pub async fn full_text_search(
        pool: &PgPool,
        collection_id: Uuid,
        query: &str,
        language: &str,
        limit: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as(
            r#"
            SELECT chunks.id, document_id, text, language::text
            FROM chunks
            JOIN documents ON documents.id = document_id
            WHERE documents.collection_id = $1
            ORDER BY ts_rank_cd(fts_vector, plainto_tsquery($2::regconfig, $3)) DESC
            LIMIT $4
            "#,
        )
        .bind(collection_id)
        .bind(language)
        .bind(query)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

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
            WHERE documents.collection_id = $1
            ORDER BY $2 <=> embedding
            LIMIT $3
            "#,
        )
        .bind(collection_id)
        .bind(embedding)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}
