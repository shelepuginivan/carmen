use std::path::Path;

use uuid::Uuid;

mod drivers;
mod env;
mod error;
mod stream;

pub use drivers::*;
pub use error::*;
pub use stream::*;

use crate::env::read_env;

pub(crate) const EXPORTED_DOCUMENTS_PREFIX: &str = "exported";
pub(crate) const RAW_DOCUMENTS_PREFIX: &str = "raw";

pub struct Storage {
    driver: Driver,
}

impl Storage {
    pub fn new_from_env() -> Result<Self> {
        let driver_type = read_env("CARMEN_STORAGE_DRIVER")?;

        let driver = match driver_type.as_str() {
            "fs" => drivers::fs::FS::new_from_env()?.into(),
            "s3" => drivers::s3::S3::new_from_env()?.into(),
            _ => return Err(Error::UnknownDriver(driver_type)),
        };

        Ok(Self { driver })
    }

    pub async fn get_exported_document_as_string(&self, id: Uuid) -> Result<String> {
        self.driver
            .get_object_as_string(&format!("{EXPORTED_DOCUMENTS_PREFIX}/{id}"))
            .await
    }

    pub async fn get_exported_document_as_stream(&self, id: Uuid) -> Result<Stream> {
        self.driver
            .get_object_as_stream(&format!("{EXPORTED_DOCUMENTS_PREFIX}/{id}"))
            .await
    }

    pub async fn get_raw_document_as_stream(&self, id: Uuid) -> Result<Stream> {
        self.driver
            .get_object_as_stream(&format!("{RAW_DOCUMENTS_PREFIX}/{id}"))
            .await
    }

    pub async fn put_exported_document_from_file(&self, id: Uuid, path: &Path) -> Result<()> {
        self.driver
            .put_object_from_local_file(&format!("{EXPORTED_DOCUMENTS_PREFIX}/{id}"), path)
            .await
    }

    pub async fn put_raw_document_from_file(&self, id: Uuid, path: &Path) -> Result<()> {
        self.driver
            .put_object_from_local_file(&format!("{RAW_DOCUMENTS_PREFIX}/{id}"), path)
            .await
    }

    pub async fn delete_document(&self, id: Uuid) -> Result<()> {
        self.driver
            .delete_many_objects(&[
                format!("{EXPORTED_DOCUMENTS_PREFIX}/{id}"),
                format!("{RAW_DOCUMENTS_PREFIX}/{id}"),
            ])
            .await
    }

    pub async fn delete_documents(&self, ids: &[Uuid]) -> Result<()> {
        let mut objects = Vec::with_capacity(2 * ids.len());

        for id in ids {
            objects.push(format!("{EXPORTED_DOCUMENTS_PREFIX}/{id}"));
            objects.push(format!("{RAW_DOCUMENTS_PREFIX}/{id}"));
        }

        self.driver.delete_many_objects(&objects).await
    }
}
