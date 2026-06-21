use fastembed::{RerankInitOptions, RerankerModel, TextRerank};

use crate::env::read_env;
use crate::error::Result;

pub trait Rerankable {
    fn content(&self) -> &str;
}

pub struct Reranker {
    model: TextRerank,
    batch_size: Option<usize>,
}

impl Reranker {
    pub fn new_from_env() -> Result<Self> {
        let model = read_env("CARMEN_RERANKER_MODEL")?.unwrap_or(RerankerModel::BGERerankerBase);
        let intra_threads = read_env("CARMEN_RERANKER_THREADS")?;
        let batch_size = read_env("CARMEN_RERANKER_BATCH_SIZE")?;

        let mut options = RerankInitOptions::new(model);

        if let Some(threads) = intra_threads {
            options = options.with_intra_threads(threads);
        }

        let model = TextRerank::try_new(options)?;

        Ok(Self { model, batch_size })
    }

    pub fn rerank(&mut self, query: &str, chunks: &mut [impl Rerankable]) -> Result<()> {
        let documents: Vec<_> = chunks.iter().map(Rerankable::content).collect();
        let mut indices: Vec<_> = self
            .model
            .rerank(query, documents, false, self.batch_size)?
            .into_iter()
            .map(|res| res.index)
            .collect();

        for idx in 0..chunks.len() {
            if indices[idx] != idx {
                let mut current_idx = idx;

                loop {
                    let target_idx = indices[current_idx];
                    indices[current_idx] = current_idx;
                    if indices[target_idx] == target_idx {
                        break;
                    }
                    chunks.swap(current_idx, target_idx);
                    current_idx = target_idx;
                }
            }
        }

        Ok(())
    }
}
