use std::env;

use s3::creds::Credentials;
use s3::{Bucket, Region};

use crate::StorageError;

pub struct Storage {
    bucket: Box<Bucket>,
}

impl Storage {
    pub fn new_from_env() -> Result<Self, StorageError> {
        let endpoint = read_env("CARMEN_S3_ENDPOINT")?;
        let region = read_env("CARMEN_S3_REGION")?;
        let bucket = read_env("CARMEN_S3_BUCKET")?;
        let access_key = read_env("CARMEN_S3_ACCESS_KEY")?;
        let secret_key = read_env("CARMEN_S3_SECRET_KEY")?;

        let region = Region::Custom { region, endpoint };
        let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;
        let bucket = Bucket::new(&bucket, region, credentials)?;

        Ok(Self { bucket })
    }
}

fn read_env(key: &'static str) -> Result<String, StorageError> {
    match env::var(key) {
        Ok(var) => Ok(var),
        Err(_) => Err(StorageError::Environment(key)),
    }
}
