use carmen_db::collections::{CollectionExtraction, CollectionExtractionType};
use carmen_db::documents::Document;
use s3::Bucket;
use s3::serde_types::ObjectIdentifier;
use sqlx::PgPool;
use tokio::fs::File;
use uuid::Uuid;

use super::{AddedDocument, DocumentDiff, UpdatedDocument};

const RAW_DOCUMENTS_PREFIX: &str = "raw";
const EXTRACTED_DOCUMENTS_PREFIX: &str = "extracted";

pub struct DocumentUpdater<'a> {
    pool: &'a PgPool,
    bucket: &'a Bucket,
}

impl<'a> DocumentUpdater<'a> {
    pub fn new(pool: &'a PgPool, bucket: &'a Bucket) -> Self {
        Self { pool, bucket }
    }

    pub async fn update(
        &self,
        extraction: &CollectionExtraction,
        diff: &DocumentDiff,
    ) -> anyhow::Result<()> {
        if extraction.extraction_type == CollectionExtractionType::Override {
            self.remove_documents(&diff.removed).await?;
        }

        for doc in diff.added.iter() {
            self.add_document(extraction.collection_id, doc).await?;
        }

        for doc in diff.updated.iter() {
            self.update_document(doc).await?;
        }

        Ok(())
    }

    async fn add_document(&self, collection_id: Uuid, doc: &AddedDocument) -> anyhow::Result<()> {
        let new_document =
            Document::insert(self.pool, collection_id, &doc.canonical_path, doc.checksum).await?;
        Document::schedule_indexing(self.pool, new_document.id).await?;

        let mut raw_file = File::open(&doc.raw_path).await?;
        self.bucket
            .put_object_stream(
                &mut raw_file,
                format!("{RAW_DOCUMENTS_PREFIX}/{}", new_document.id),
            )
            .await?;

        let mut exported_file = File::open(&doc.exported_path).await?;
        self.bucket
            .put_object_stream(
                &mut exported_file,
                format!("{EXTRACTED_DOCUMENTS_PREFIX}/{}", new_document.id),
            )
            .await?;

        Ok(())
    }

    async fn update_document(&self, doc: &UpdatedDocument) -> anyhow::Result<()> {
        Document::update_checksum(self.pool, doc.id, doc.checksum).await?;
        Document::schedule_indexing(self.pool, doc.id).await?;

        let mut raw_file = File::open(&doc.raw_path).await?;
        self.bucket
            .put_object_stream(&mut raw_file, format!("{RAW_DOCUMENTS_PREFIX}/{}", doc.id))
            .await?;

        let mut exported_file = File::open(&doc.exported_path).await?;
        self.bucket
            .put_object_stream(
                &mut exported_file,
                format!("{EXTRACTED_DOCUMENTS_PREFIX}/{}", doc.id),
            )
            .await?;

        Ok(())
    }

    async fn remove_documents(&self, docs: &[Uuid]) -> anyhow::Result<()> {
        let mut objects = Vec::with_capacity(2 * docs.len());

        for id in docs {
            Document::delete(self.pool, *id).await?;

            objects.push(ObjectIdentifier::new(format!(
                "{RAW_DOCUMENTS_PREFIX}/{id}"
            )));
            objects.push(ObjectIdentifier::new(format!(
                "{EXTRACTED_DOCUMENTS_PREFIX}/{id}"
            )));
        }

        self.bucket.delete_objects(objects).await?;

        Ok(())
    }
}
