use std::path::Path;

use enum_dispatch::enum_dispatch;

use crate::error::Result;

// TODO: come up with a solution for byte stream responses.
type ResponseDataStream = s3::request::ResponseDataStream;

#[enum_dispatch]
pub trait StorageDriver {
    async fn get_object_as_string(&self, id: &str) -> Result<String>;
    async fn get_object_as_stream(&self, id: &str) -> Result<ResponseDataStream>;
    async fn put_object_from_local_file(&self, id: &str, path: &Path) -> Result<()>;
    async fn delete_object(&self, id: &str) -> Result<()>;
    async fn delete_many_objects(&self, ids: &[&str]) -> Result<()>;
}

#[enum_dispatch(StorageDriver)]
pub enum Driver {}
