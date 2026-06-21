use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use crate::env::read_env;
use crate::error::Result;

pub struct Embedder {
    model: TextEmbedding,
    batch_size: Option<usize>,
    query_prefix: String,
    chunk_prefix: String,
}

impl Embedder {
    pub fn new_from_env() -> Result<Self> {
        let model = read_env("CARMEN_EMBEDDING_MODEL")?.unwrap_or(EmbeddingModel::AllMiniLML6V2);
        let intra_threads = read_env("CARMEN_EMBEDDING_THREADS")?;
        let batch_size = read_env("CARMEN_EMBEDDING_BATCH_SIZE")?;

        let query_prefix =
            read_env("CARMEN_EMBEDDING_PREFIX_QUERY")?.unwrap_or_else(|| "query: ".to_owned());
        let chunk_prefix =
            read_env("CARMEN_EMBEDDING_PREFIX_CHUNK")?.unwrap_or_else(|| "passage: ".to_owned());

        let mut options = InitOptions::new(model);

        if let Some(threads) = intra_threads {
            options = options.with_intra_threads(threads);
        }

        let model = TextEmbedding::try_new(options)?;

        Ok(Self {
            model,
            batch_size,
            query_prefix,
            chunk_prefix,
        })
    }

    pub fn embed<S>(&mut self, s: impl AsRef<[S]>) -> Result<Vec<Vec<f32>>>
    where
        S: AsRef<str> + Send + Sync,
    {
        Ok(self.model.embed(s, self.batch_size)?)
    }

    pub fn embed_query(&mut self, query: &str) -> Result<Vec<f32>> {
        let prefixed = format!("{}{query}", self.query_prefix);
        Ok(self.model.embed([prefixed], self.batch_size)?.remove(0))
    }

    pub fn embed_chunks(&mut self, s: &[&str]) -> Result<Vec<Vec<f32>>> {
        let chunks: Vec<_> = s
            .into_iter()
            .map(|s| format!("{}{s}", self.chunk_prefix))
            .collect();

        Ok(self.model.embed(chunks, self.batch_size)?)
    }
}
