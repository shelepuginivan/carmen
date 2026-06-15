use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use carmen_db::collections::CollectionExtraction;
use enum_dispatch::enum_dispatch;
mod git;
use git::GitDownloader;

#[derive(Default, strum::Display)]
pub enum DocumentFormat {
    #[default]
    #[strum(to_string = "plain")]
    PlainText,
    #[strum(to_string = "gfm")]
    Markdown,
    #[strum(to_string = "rst")]
    ReStructuredText,
}

impl DocumentFormat {
    pub fn guess_for_path(path: &Path) -> Option<Self> {
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => return None,
        };

        match extension {
            "md" => Some(Self::Markdown),
            "txt" => Some(Self::PlainText),
            "rst" => Some(Self::ReStructuredText),

            _ => None,
        }
    }
}

pub struct DownloadedDocument {
    pub file_path: PathBuf,
    pub canonical_path: String,
    pub format: DocumentFormat,
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
