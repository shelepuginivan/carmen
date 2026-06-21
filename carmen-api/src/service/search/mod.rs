use std::sync::{Arc, Mutex};

use carmen_db::chunks::Chunk;
use carmen_nlp::{Embedder, LangDetector};
use sqlx::PgPool;

use crate::service::search::dto::SearchParameters;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct SearchService {
    pool: Arc<PgPool>,
    embedder: Arc<Mutex<Embedder>>,
    detector: Arc<LangDetector>,
}

impl SearchService {
    pub fn new(
        pool: Arc<PgPool>,
        embedder: Arc<Mutex<Embedder>>,
        detector: Arc<LangDetector>,
    ) -> Self {
        Self {
            pool,
            embedder,
            detector,
        }
    }

    pub async fn full_text(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let language = self.detector.detect(&params.query).to_string();

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
        let embedding = self.embedder.lock().unwrap().embed_query(&params.query)?;

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

    pub async fn hybrid(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let language = self.detector.detect(&params.query).to_string();
        let embedding = self.embedder.lock().unwrap().embed_query(&params.query)?;

        Ok(Chunk::hybrid_search(
            &self.pool,
            params.collection,
            &params.query,
            &language,
            embedding,
            params.limit.unwrap_or(10).into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect())
    }
}
