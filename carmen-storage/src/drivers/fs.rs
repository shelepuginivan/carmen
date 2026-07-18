use std::path::{Path, PathBuf};

use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;

use crate::drivers::StorageDriver;
use crate::env::read_env;
use crate::error::{Error, Result};
use crate::stream::Stream;

pub struct FS {
    root: PathBuf,
}

impl FS {
    pub fn new_from_env() -> Result<Self> {
        let root: PathBuf = read_env("CARMEN_STORAGE_FS_ROOT")?.into();

        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }

        Ok(Self { root })
    }

    fn resolve_path(&self, id: &str) -> Result<PathBuf> {
        let path = self.root.join(id);

        if !path.starts_with(&self.root) {
            Err(Error::NotFound)
        } else {
            Ok(path)
        }
    }
}

impl StorageDriver for FS {
    async fn get_object_as_string(&self, id: &str) -> Result<String> {
        let path = self.resolve_path(id)?;
        let s = fs::read_to_string(path).await?;
        Ok(s)
    }

    async fn get_object_as_stream(&self, id: &str) -> Result<Stream> {
        let path = self.resolve_path(id)?;
        let file = File::open(path).await?;
        let stream = ReaderStream::new(file);
        Ok(Stream::FS(stream))
    }

    async fn put_object_from_local_file(&self, id: &str, path: &Path) -> Result<()> {
        let dest = self.resolve_path(id)?;
        fs::copy(path, dest).await?;
        Ok(())
    }

    async fn delete_many_objects(&self, ids: &[String]) -> Result<()> {
        for id in ids {
            let path = self.resolve_path(id)?;
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}
