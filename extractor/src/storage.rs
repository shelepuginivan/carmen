use s3::creds::Credentials;
use s3::{Bucket, Region};

use crate::config::Config;

pub struct DocumentStorage {
    bucket: Box<Bucket>,
}

impl DocumentStorage {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let region = Region::Custom {
            region: cfg.s3_region.clone(),
            endpoint: cfg.s3_endpoint.clone(),
        };

        let credentials = Credentials::new(
            Some(&cfg.s3_access_key),
            Some(&cfg.s3_secret_key),
            None,
            None,
            None,
        )?;

        let bucket = Bucket::new(&cfg.s3_bucket, region, credentials)?.with_path_style();

        Ok(Self { bucket })
    }

    pub async fn get_document(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        Ok(self.bucket.get_object(key).await?.into_bytes().to_vec())
    }
}
