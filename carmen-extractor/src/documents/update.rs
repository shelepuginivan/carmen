use carmen_db::documents::Document;
use s3::Bucket;
use sqlx::PgPool;
use tokio::fs::File;
use uuid::Uuid;

use super::{AddedDocument, DocumentDiff, UpdatedDocument};

const EXTRACTED_DOCUMENTS_PREFIX: &str = "extracted";

pub struct DocumentUpdater {
    pool: PgPool,
    bucket: Box<Bucket>,
}

impl DocumentUpdater {
    pub fn new(pool: PgPool, bucket: Box<Bucket>) -> Self {
        Self { pool, bucket }
    }

    pub async fn update(&self, collection_id: Uuid, diff: DocumentDiff) -> anyhow::Result<()> {
        for id in diff.removed {
            self.remove_document(id).await?;
        }

        for doc in diff.added {
            self.add_document(collection_id, doc).await?;
        }

        for doc in diff.updated {
            self.update_document(doc).await?;
        }

        Ok(())
    }

    async fn add_document(&self, collection_id: Uuid, doc: AddedDocument) -> anyhow::Result<()> {
        let new_document =
            Document::insert(&self.pool, collection_id, &doc.canonical_path, doc.checksum).await?;

        let mut file = File::open(doc.file_path).await?;

        self.bucket
            .put_object_stream(
                &mut file,
                format!("{EXTRACTED_DOCUMENTS_PREFIX}/{}", new_document.id),
            )
            .await?;

        Ok(())
    }

    async fn update_document(&self, doc: UpdatedDocument) -> anyhow::Result<()> {
        Document::update_checksum(&self.pool, doc.id, doc.checksum).await?;

        let mut file = File::open(doc.file_path).await?;

        self.bucket
            .put_object_stream(
                &mut file,
                format!("{EXTRACTED_DOCUMENTS_PREFIX}/{}", doc.id),
            )
            .await?;

        Ok(())
    }

    async fn remove_document(&self, id: Uuid) -> anyhow::Result<()> {
        Document::delete(&self.pool, id).await?;
        self.bucket
            .delete_object(format!("{EXTRACTED_DOCUMENTS_PREFIX}/{id}"))
            .await?;

        Ok(())
    }
}
