use std::path::Path;

use bytes::Bytes;
use enum_dispatch::enum_dispatch;
use futures_core::TryStream;

use crate::error::Result;

mod fs;
mod s3;

use fs::FS;
use s3::S3;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[enum_dispatch]
pub trait StorageDriver {
    async fn get_object_as_string(&self, id: &str) -> Result<String>;
    async fn get_object_as_stream(
        &self,
        id: &str,
    ) -> Result<impl TryStream<Ok: Into<Bytes>, Error: Into<BoxError>>>;
    async fn put_object_from_local_file(&self, id: &str, path: &Path) -> Result<()>;
    async fn delete_many_objects(&self, ids: &[&str]) -> Result<()>;
}

#[enum_dispatch(StorageDriver)]
pub enum Driver {
    S3,
}
