use carmen_db::documents::Document;
use carmen_db::extractions::{Extraction, ExtractionType};
use carmen_db::indexing::Indexing;
use carmen_s3::Storage;
use sqlx::PgPool;
use uuid::Uuid;

use super::{AddedDocument, DocumentDiff, UpdatedDocument};

pub struct DocumentUpdater<'a> {
    pool: &'a PgPool,
    storage: &'a Storage,
}

impl<'a> DocumentUpdater<'a> {
    pub fn new(pool: &'a PgPool, storage: &'a Storage) -> Self {
        Self { pool, storage }
    }

    pub async fn update(&self, extraction: &Extraction, diff: &DocumentDiff) -> anyhow::Result<()> {
        if extraction.extraction_type == ExtractionType::Overwrite {
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
        Indexing::schedule(self.pool, new_document.id).await?;

        self.storage
            .put_raw_document_from_path(new_document.id, &doc.raw_path)
            .await?;

        self.storage
            .put_exported_document_from_path(new_document.id, &doc.exported_path)
            .await?;

        Ok(())
    }

    async fn update_document(&self, doc: &UpdatedDocument) -> anyhow::Result<()> {
        Document::update_checksum(self.pool, doc.id, doc.checksum).await?;
        Indexing::schedule(self.pool, doc.id).await?;

        self.storage
            .put_raw_document_from_path(doc.id, &doc.raw_path)
            .await?;

        self.storage
            .put_exported_document_from_path(doc.id, &doc.exported_path)
            .await?;

        Ok(())
    }

    async fn remove_documents(&self, docs: &[Uuid]) -> anyhow::Result<()> {
        for id in docs {
            Document::delete(self.pool, *id).await?;
        }
        Ok(self.storage.delete_documents(docs).await?)
    }
}
