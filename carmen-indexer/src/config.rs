use std::env;
use std::str::FromStr;

use anyhow::anyhow;
use fastembed::EmbeddingModel;
use lingua::Language;

pub struct Config {
    pub embedding_threads: Option<usize>,
    pub embedding_model: EmbeddingModel,
    pub max_chunk_size: usize,
    pub languages: Vec<Language>,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
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
            1024
        };

        let languages = if let Ok(v) = env::var("CARMEN_DETECT_LANGUAGES") {
            v.split(',')
                .map(Language::from_str)
                .collect::<Result<_, _>>()?
        } else {
            vec![
                Language::Arabic,
                Language::Chinese,
                Language::English,
                Language::French,
                Language::German,
                Language::Japanese,
                Language::Portuguese,
                Language::Russian,
                Language::Spanish,
            ]
        };

        Ok(Self {
            embedding_model,
            embedding_threads,
            max_chunk_size,
            languages,
        })
    }
}
