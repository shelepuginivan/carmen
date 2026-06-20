use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

use crate::env::read_env;
use crate::error::Result;

pub struct Embedder {
    model: TextEmbedding,
    batch_size: Option<usize>,
}

impl Embedder {
    pub fn new_from_env() -> Result<Self> {
        let model = read_env("CARMEN_EMBEDDING_MODEL")?.unwrap_or(EmbeddingModel::AllMiniLML6V2);
        let intra_threads = read_env("CARMEN_EMBEDDING_THREADS")?;
        let batch_size = read_env("CARMEN_EMBEDDING_BATCH_SIZE")?;

        let mut options = InitOptions::new(model);

        if let Some(threads) = intra_threads {
            options = options.with_intra_threads(threads);
        }

        let model = TextEmbedding::try_new(options)?;

        Ok(Self { model, batch_size })
    }

    pub fn embed<S>(&mut self, s: impl AsRef<[S]>) -> Result<Vec<Vec<f32>>>
    where
        S: AsRef<str> + Send + Sync,
    {
        Ok(self.model.embed(s, self.batch_size)?)
    }
}
