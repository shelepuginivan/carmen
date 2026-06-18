use std::sync::{Arc, Mutex};

use carmen_db::chunks::Chunk;
use fastembed::TextEmbedding;
use lingua::LanguageDetector;
use sqlx::PgPool;

use crate::service::search::dto::SearchParameters;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct SearchService {
    pool: Arc<PgPool>,
    embedder: Arc<Mutex<TextEmbedding>>,
    detector: Arc<LanguageDetector>,
}

impl SearchService {
    pub fn new(
        pool: Arc<PgPool>,
        embedder: Arc<Mutex<TextEmbedding>>,
        detector: Arc<LanguageDetector>,
    ) -> Self {
        Self {
            pool,
            embedder,
            detector,
        }
    }

    pub async fn full_text(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let language = self
            .detector
            .detect_language_of(&params.query)
            .map(|lang| lang.to_string())
            .unwrap_or_else(|| "simple".to_string());

        Ok(Chunk::full_text_search(
            &self.pool,
            params.collection,
            &params.query,
            &language,
            params.limit.unwrap_or(10).into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect())
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
