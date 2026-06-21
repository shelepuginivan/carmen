use std::env;

pub struct Config {
    pub max_chunk_size: usize,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let max_chunk_size = if let Ok(v) = env::var("CARMEN_INDEXER_MAX_CHUNK_SIZE") {
            v.parse()?
        } else {
            1024
        };

        Ok(Self { max_chunk_size })
    }
}
