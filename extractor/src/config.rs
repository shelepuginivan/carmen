use std::env;

pub struct Config {
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl Config {
    pub fn read_from_env() -> anyhow::Result<Self> {
        let s3_endpoint = env::var("CARMEN_EXTRACTOR_S3_ENDPOINT")?;
        let s3_region = env::var("CARMEN_EXTRACTOR_S3_REGION")?;
        let s3_bucket = env::var("CARMEN_EXTRACTOR_S3_BUCKET")?;
        let s3_access_key = env::var("CARMEN_EXTRACTOR_S3_ACCESS_KEY")?;
        let s3_secret_key = env::var("CARMEN_EXTRACTOR_S3_SECRET_KEY")?;

        Ok(Self {
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
        })
    }
}
