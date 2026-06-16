use std::env;
use std::str::FromStr;

use anyhow::anyhow;
use fastembed::EmbeddingModel;
use tokio::sync::Semaphore;

pub struct Config {
    pub task_limit: usize,

    pub embedding_threads: Option<usize>,
    pub embedding_model: EmbeddingModel,
    pub max_chunk_size: usize,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let task_limit = if let Ok(v) = env::var("CARMEN_INDEXER_TASK_LIMIT") {
            usize::from_str(&v)?.clamp(1, Semaphore::MAX_PERMITS)
        } else {
            3
        };

        let embedding_threads = if let Ok(v) = env::var("CARMEN_INDEXER_EMBEDDING_THREADS") {
            Some(v.parse()?)
        } else {
            None
        };

        let embedding_model = if let Ok(v) = env::var("CARMEN_EMBEDDING_MODEL") {
            v.parse().map_err(|s: String| anyhow!(s))?
        } else {
            EmbeddingModel::AllMiniLML6V2
        };

        let max_chunk_size = if let Ok(v) = env::var("CARMEN_INDEXER_MAX_CHUNK_SIZE") {
            v.parse()?
        } else {
            512
        };

        Ok(Self {
            task_limit,
            embedding_model,
            embedding_threads,
            max_chunk_size,
        })
    }
}
