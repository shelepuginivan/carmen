macro_rules! cfg_env {
    ($name:literal) => {
        std::env::var(concat!("CARMEN_EXTRACTOR_", $name))
    };
}

pub struct Config {
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl Config {
    pub fn read_from_env() -> anyhow::Result<Self> {
        let s3_endpoint = cfg_env!("S3_ENDPOINT")?;
        let s3_region = cfg_env!("S3_REGION")?;
        let s3_bucket = cfg_env!("S3_BUCKET")?;
        let s3_access_key = cfg_env!("S3_ACCESS_KEY")?;
        let s3_secret_key = cfg_env!("S3_SECRET_KEY")?;

        Ok(Self {
            s3_endpoint,
            s3_region,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
        })
    }
}
