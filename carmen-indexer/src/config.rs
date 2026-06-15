use std::env;
use std::str::FromStr;

use tokio::sync::Semaphore;

pub struct Config {
    pub postgres_url: String,

    pub task_limit: usize,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let postgres_url = env::var("CARMEN_POSTGRES_URL")?;

        let task_limit = if let Ok(v) = env::var("CARMEN_INDEXER_TASK_LIMIT") {
            usize::from_str(&v)?.clamp(1, Semaphore::MAX_PERMITS)
        } else {
            3
        };

        Ok(Self {
            postgres_url,
            task_limit,
        })
    }
}
