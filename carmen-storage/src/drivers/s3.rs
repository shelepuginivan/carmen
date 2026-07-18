use std::path::Path;

use s3::creds::Credentials;
use s3::serde_types::ObjectIdentifier;
use s3::{Bucket, Region};
use tokio::fs::File;

use crate::drivers::StorageDriver;
use crate::env::read_env;
use crate::error::Result;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub struct S3 {
    bucket: Box<Bucket>,
}

impl S3 {
    pub fn new_from_env() -> Result<Self> {
        let endpoint = read_env("CARMEN_STORAGE_S3_ENDPOINT")?;
        let region = read_env("CARMEN_STORAGE_S3_REGION")?;
        let bucket = read_env("CARMEN_STORAGE_S3_BUCKET")?;
        let access_key = read_env("CARMEN_STORAGE_S3_ACCESS_KEY")?;
        let secret_key = read_env("CARMEN_STORAGE_S3_SECRET_KEY")?;

        let region = Region::Custom { region, endpoint };
        let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;
        let bucket = Bucket::new(&bucket, region, credentials)?.with_path_style();

        Ok(Self { bucket })
    }
}

impl StorageDriver for S3 {
    async fn get_object_as_string(&self, id: &str) -> Result<String> {
        Ok(self.bucket.get_object(id).await?.to_string()?)
    }

    async fn get_object_as_stream(&self, id: &str) -> Result<Stream> {
        Ok(Stream::S3(self.bucket.get_object_stream(id).await?.bytes))
    }

    async fn put_object_from_local_file(&self, id: &str, path: &Path) -> Result<()> {
        let mut file = File::open(path).await?;
        self.bucket.put_object_stream(&mut file, id).await?;
        Ok(())
    }

    async fn delete_many_objects(&self, ids: &[String]) -> Result<()> {
        let objects: Vec<_> = ids.into_iter().map(ObjectIdentifier::new).collect();
        self.bucket.delete_objects(objects).await?;
        Ok(())
    }
}
