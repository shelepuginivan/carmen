use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use carmen_db::collections::CollectionExtraction;
use enum_dispatch::enum_dispatch;
use strum::EnumString;

use crate::document::Document;

mod git;
mod github_wiki;

use git::GitExtractor;
use github_wiki::GitHubWikiExtractor;

#[enum_dispatch]
pub trait Extractor {
    async fn extract(
        &self,
        extraction: &CollectionExtraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>>;
}

#[enum_dispatch(Extractor)]
pub enum ExtractorEnum {
    GitExtractor,
    GitHubWikiExtractor,
}

#[derive(PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SourceType {
    Git,
    #[strum(serialize = "github_wiki")]
    GitHubWiki,
}

pub static EXTRACTORS: LazyLock<HashMap<SourceType, ExtractorEnum>> = LazyLock::new(|| {
    HashMap::from([
        (SourceType::Git, GitExtractor.into()),
        (SourceType::GitHubWiki, GitHubWikiExtractor.into()),
    ])
});
