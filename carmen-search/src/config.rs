use std::env;
use std::str::FromStr;

use anyhow::anyhow;
use fastembed::EmbeddingModel;
use lingua::Language;

pub struct Config {
    pub http_addr: String,
    pub docs_path: Option<String>,

    pub embedding_threads: Option<usize>,
    pub embedding_model: EmbeddingModel,
    pub languages: Vec<Language>,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let http_addr =
            env::var("CARMEN_SEARCH_ADDR").unwrap_or_else(|_| "0.0.0.0:5124".to_owned());
        let docs_path = env::var("CARMEN_SEARCH_DOCS_PATH").ok();

        let embedding_threads = if let Ok(v) = env::var("CARMEN_SEARCH_EMBEDDING_THREADS") {
            Some(v.parse()?)
        } else {
            None
        };

        let embedding_model = if let Ok(v) = env::var("CARMEN_EMBEDDING_MODEL") {
            v.parse().map_err(|s: String| anyhow!(s))?
        } else {
            EmbeddingModel::AllMiniLML6V2
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
            http_addr,
            docs_path,
            embedding_model,
            embedding_threads,
            languages,
        })
    }
}
