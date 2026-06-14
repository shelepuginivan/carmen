use std::sync::LazyLock;

use carmen_db::collections::Collection;
use enum_dispatch::enum_dispatch;

mod git;
use git::GitExtractor;

#[enum_dispatch]
pub trait Extractor {
    fn can_extract(&self, collection: &Collection) -> bool;
    async fn extract(&self, collection: &Collection) -> anyhow::Result<()>;
}

#[enum_dispatch(Extractor)]
pub enum ExtractorEnum {
    GitExtractor,
}

pub static EXTRACTORS: LazyLock<Vec<ExtractorEnum>> = LazyLock::new(|| vec![GitExtractor.into()]);
