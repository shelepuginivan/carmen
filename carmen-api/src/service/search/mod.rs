use std::sync::{Arc, Mutex};

use carmen_db::chunks::Chunk;
use carmen_nlp::{Embedder, LangDetector, Reranker};
use sqlx::PgPool;

use crate::service::search::dto::SearchParameters;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct SearchService {
    pool: PgPool,
    embedder: Arc<Mutex<Embedder>>,
    detector: Arc<LangDetector>,
    reranker: Arc<Mutex<Reranker>>,
}

impl SearchService {
    pub fn new(
        pool: PgPool,
        embedder: Arc<Mutex<Embedder>>,
        detector: Arc<LangDetector>,
        reranker: Arc<Mutex<Reranker>>,
    ) -> Self {
        Self {
            pool,
            embedder,
            detector,
            reranker,
        }
    }

    pub async fn full_text(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let language = self.detector.detect(&params.query).to_string();

        let mut chunks = Chunk::full_text_search(
            &self.pool,
            params.collection,
            &params.query,
            &language,
            params.limit.into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect();

        self.rerank(&mut chunks, params)?;

        Ok(chunks)
    }

    pub async fn semantic(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let embedding = self.embedder.lock().unwrap().embed_query(&params.query)?;

        let mut chunks = Chunk::semantic_search(
            &self.pool,
            params.collection,
            embedding,
            params.limit.into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect();

        self.rerank(&mut chunks, params)?;

        Ok(chunks)
    }

    pub async fn hybrid(&self, params: SearchParameters) -> Result<Vec<dto::Chunk>> {
        let language = self.detector.detect(&params.query).to_string();
        let embedding = self.embedder.lock().unwrap().embed_query(&params.query)?;

        let mut chunks = Chunk::hybrid_search(
            &self.pool,
            params.collection,
            &params.query,
            &language,
            embedding,
            params.limit.into(),
        )
        .await?
        .into_iter()
        .map(dto::Chunk::from)
        .collect();

        self.rerank(&mut chunks, params)?;

        Ok(chunks)
    }

    fn rerank(&self, chunks: &mut Vec<dto::Chunk>, params: SearchParameters) -> Result<()> {
        if let Some(rerank_limit) = params.rerank {
            self.reranker
                .lock()
                .unwrap()
                .rerank(&params.query, chunks)?;

            chunks.truncate(rerank_limit.into());
        }

        Ok(())
    }
}
