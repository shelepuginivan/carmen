use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use carmen_db::collections::CollectionExtraction;
use enum_dispatch::enum_dispatch;

mod git;
use git::GitDownloader;

pub struct DownloadedDocument {
    pub file_path: PathBuf,
    pub canonical_path: String,
}

#[enum_dispatch]
pub trait Downloader {
    fn can_download(&self, extraction: &CollectionExtraction) -> bool;
    async fn download(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<DownloadedDocument>>;
}

#[enum_dispatch(Downloader)]
pub enum DownloaderEnum {
    GitDownloader,
}

pub static DOWNLOADERS: LazyLock<Vec<DownloaderEnum>> =
    LazyLock::new(|| vec![GitDownloader.into()]);
