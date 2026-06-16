use std::env;
use std::path::Path;

use s3::creds::Credentials;
use s3::serde_types::ObjectIdentifier;
use s3::{Bucket, Region};
use tokio::fs::File;
use uuid::Uuid;

use crate::{Result, StorageError};

const RAW_DOCUMENTS_PREFIX: &str = "raw";
const EXPORTED_DOCUMENTS_PREFIX: &str = "exported";

macro_rules! raw_document {
    ($key:expr) => {
        format!("{RAW_DOCUMENTS_PREFIX}/{}", $key)
    };
}

macro_rules! exported_document {
    ($key:expr) => {
        format!("{EXPORTED_DOCUMENTS_PREFIX}/{}", $key)
    };
}

#[derive(Clone, Debug)]
pub struct Storage {
    bucket: Box<Bucket>,
}

impl Storage {
    pub fn new_from_env() -> Result<Self> {
        let endpoint = read_env("CARMEN_S3_ENDPOINT")?;
        let region = read_env("CARMEN_S3_REGION")?;
        let bucket = read_env("CARMEN_S3_BUCKET")?;
        let access_key = read_env("CARMEN_S3_ACCESS_KEY")?;
        let secret_key = read_env("CARMEN_S3_SECRET_KEY")?;

        let region = Region::Custom { region, endpoint };
        let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;
        let bucket = Bucket::new(&bucket, region, credentials)?.with_path_style();

        Ok(Self { bucket })
    }

    pub async fn get_exported_document_as_string(&self, id: Uuid) -> Result<String> {
        Ok(self
            .bucket
            .get_object(exported_document!(id))
            .await
            .unwrap()
            .to_string()?)
    }

    pub async fn put_raw_document_from_path(&self, id: Uuid, path: &Path) -> Result<()> {
        let mut file = File::open(path).await?;

        Ok(self
            .bucket
            .put_object_stream(&mut file, raw_document!(id))
            .await
            .map(|_| ())?)
    }

    pub async fn put_exported_document_from_path(&self, id: Uuid, path: &Path) -> Result<()> {
        let mut file = File::open(path).await?;

        Ok(self
            .bucket
            .put_object_stream(&mut file, exported_document!(id))
            .await
            .map(|_| ())?)
    }

    pub async fn delete_document(&self, id: Uuid) -> Result<()> {
        Ok(self
            .bucket
            .delete_objects(vec![
                ObjectIdentifier::new(raw_document!(id)),
                ObjectIdentifier::new(exported_document!(id)),
            ])
            .await
            .map(|_| ())?)
    }

    pub async fn delete_documents(&self, ids: &[Uuid]) -> Result<()> {
        let mut objects = Vec::with_capacity(2 * ids.len());

        for id in ids {
            objects.push(ObjectIdentifier::new(raw_document!(id)));
            objects.push(ObjectIdentifier::new(exported_document!(id)));
        }

        Ok(self.bucket.delete_objects(objects).await.map(|_| ())?)
    }
}

fn read_env(key: &'static str) -> Result<String, StorageError> {
    match env::var(key) {
        Ok(var) => Ok(var),
        Err(_) => Err(StorageError::Environment(key)),
    }
}
