use std::env;

pub struct Config {
    pub postgres_url: String,

    pub http_addr: String,
    pub docs_path: Option<String>,

    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl Config {
    pub fn load_env() -> anyhow::Result<Self> {
        let postgres_url = env::var("CARMEN_POSTGRES_URL")?;

        let http_addr = env::var("CARMEN_ADDR").unwrap_or_else(|_| "0.0.0.0:5124".to_owned());
        let docs_path = env::var("CARMEN_DOCS_PATH").ok();

        let s3_endpoint = env::var("CARMEN_S3_ENDPOINT")?;
        let s3_region = env::var("CARMEN_S3_REGION")?;
        let s3_bucket = env::var("CARMEN_S3_BUCKET")?;
        let s3_access_key = env::var("CARMEN_S3_ACCESS_KEY")?;
        let s3_secret_key = env::var("CARMEN_S3_SECRET_KEY")?;

        Ok(Self {
            http_addr,
            postgres_url,
            docs_path,
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
        })
    }
}
