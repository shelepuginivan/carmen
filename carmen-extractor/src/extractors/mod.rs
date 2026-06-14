use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use carmen_db::collections::Collection;
use enum_dispatch::enum_dispatch;

mod git;
use git::GitExtractor;

pub struct ExtractedDocument {
    pub file_path: PathBuf,
    pub canonical_path: String,
}

#[enum_dispatch]
pub trait Extractor {
    fn can_extract(&self, collection: &Collection) -> bool;
    async fn extract(
        &self,
        collection: &Collection,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<ExtractedDocument>>;
}

#[enum_dispatch(Extractor)]
pub enum ExtractorEnum {
    GitExtractor,
}

pub static EXTRACTORS: LazyLock<Vec<ExtractorEnum>> = LazyLock::new(|| vec![GitExtractor.into()]);
