use std::path::Path;

use enum_dispatch::enum_dispatch;

use crate::error::Result;
use crate::stream::Stream;

pub mod fs;
pub mod s3;

use fs::FS;
use s3::S3;

#[enum_dispatch]
pub trait StorageDriver {
    async fn get_object_as_string(&self, id: &str) -> Result<String>;
    async fn get_object_as_stream(&self, id: &str) -> Result<Stream>;
    async fn put_object_from_local_file(&self, id: &str, path: &Path) -> Result<()>;
    async fn delete_many_objects(&self, ids: &[String]) -> Result<()>;
}

#[enum_dispatch(StorageDriver)]
pub enum Driver {
    FS,
    S3,
}
