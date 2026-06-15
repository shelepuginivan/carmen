use std::env;
use std::str::FromStr;

use tokio::sync::Semaphore;

pub struct Config {
    pub postgres_url: String,

    pub task_limit: usize,

    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let postgres_url = env::var("CARMEN_POSTGRES_URL")?;

        let task_limit = if let Ok(v) = env::var("CARMEN_INDEXER_TASK_LIMIT") {
            usize::from_str(&v)?.clamp(1, Semaphore::MAX_PERMITS)
        } else {
            3
        };

        let s3_endpoint = env::var("CARMEN_S3_ENDPOINT")?;
        let s3_region = env::var("CARMEN_S3_REGION")?;
        let s3_bucket = env::var("CARMEN_S3_BUCKET")?;
        let s3_access_key = env::var("CARMEN_S3_ACCESS_KEY")?;
        let s3_secret_key = env::var("CARMEN_S3_SECRET_KEY")?;

        Ok(Self {
            postgres_url,
            task_limit,
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
        })
    }
}
