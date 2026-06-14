use carmen_db::collections::Collection;

use super::Extractor;

pub struct GitExtractor;

impl Extractor for GitExtractor {
    fn can_extract(&self, collection: &Collection) -> bool {
        collection.source.as_deref() == Some("git") && collection.url.is_some()
    }

    async fn extract(&self, collection: &Collection) -> anyhow::Result<()> {
        todo!()
    }
}
