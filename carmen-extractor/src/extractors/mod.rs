use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use carmen_db::extractions::Extraction;
use enum_dispatch::enum_dispatch;
use strum::EnumString;

use crate::document::Document;

mod git;
mod github_wiki;
mod man;

use git::GitExtractor;
use github_wiki::GitHubWikiExtractor;
use man::ManExtractor;

#[enum_dispatch]
pub trait Extractor {
    async fn extract(
        &self,
        extraction: &Extraction,
        tempdir: &Path,
    ) -> anyhow::Result<Vec<Document>>;
}

#[allow(clippy::enum_variant_names)]
#[enum_dispatch(Extractor)]
pub enum ExtractorEnum {
    GitExtractor,
    GitHubWikiExtractor,
    ManExtractor,
}

#[derive(PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SourceType {
    Git,
    #[strum(serialize = "github_wiki")]
    GitHubWiki,
    Man,
}

pub static EXTRACTORS: LazyLock<HashMap<SourceType, ExtractorEnum>> = LazyLock::new(|| {
    HashMap::from([
        (SourceType::Git, GitExtractor.into()),
        (SourceType::GitHubWiki, GitHubWikiExtractor.into()),
        (SourceType::Man, ManExtractor.into()),
    ])
});
