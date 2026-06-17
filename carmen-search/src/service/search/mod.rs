use std::sync::{Arc, Mutex};

use carmen_db::chunks::Chunk;
use fastembed::TextEmbedding;
use sqlx::PgPool;

use crate::service::search::dto::SearchParameters;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct SearchService {
    pool: Arc<PgPool>,
    embedder: Arc<Mutex<TextEmbedding>>,
}

impl SearchService {
    pub fn new(pool: Arc<PgPool>, embedder: Arc<Mutex<TextEmbedding>>) -> Self {
        Self { pool, embedder }
    }

    pub async fn semantic(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let embedding = self
            .embedder
            .lock()
            .unwrap()
            .embed(&[params.query], None)?
            .remove(0);

        Ok(Chunk::semantic_search(
            &self.pool,
            params.collection,
            embedding,
            params.limit.unwrap_or(10).into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect())
    }
}
