use std::path::Path;
use std::sync::LazyLock;

use carmen_db::collections::CollectionExtraction;
use enum_dispatch::enum_dispatch;

use crate::document::Document;

mod git;
use git::GitExtractor;

#[enum_dispatch]
pub trait Extractor {
    fn can_extract(&self, extraction: &CollectionExtraction) -> bool;

    async fn extract(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>>;
}

#[enum_dispatch(Extractor)]
pub enum ExtractorEnum {
    GitExtractor,
}

pub static EXTRACTORS: LazyLock<Vec<ExtractorEnum>> = LazyLock::new(|| vec![GitExtractor.into()]);
