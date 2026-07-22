use std::sync::Arc;

use axum::body::Body;
use carmen_db::{documents::Document, indexing::Indexing};
use carmen_storage::Storage;
use sqlx::PgPool;
use uuid::Uuid;

use crate::service::pagination::Pagination;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct DocumentsService {
    pool: PgPool,
    storage: Arc<Storage>,
}

impl DocumentsService {
    pub fn new(pool: PgPool, storage: Arc<Storage>) -> Self {
        Self { pool, storage }
    }

    pub async fn get_by_collection_id(
        &self,
        collection_id: Uuid,
        pagination: Pagination,
    ) -> Result<Vec<dto::Document>> {
        let page = pagination.page.saturating_sub(1);
        let offset = page.saturating_mul(pagination.size);

        Ok(
            Document::get_by_collection_id_pages(
                &self.pool,
                collection_id,
                pagination.size,
                offset,
            )
            .await?
            .into_iter()
            .map(dto::Document::from)
            .collect(),
        )
    }

    pub async fn get(&self, id: Uuid) -> Result<dto::Document> {
        Ok(Document::get(&self.pool, id).await?.into())
    }

    pub async fn get_raw_stream(&self, id: Uuid) -> Result<Body> {
        Ok(self.storage.get_raw_document_as_stream(id).await?.into())
    }

    pub async fn get_exported_stream(&self, id: Uuid) -> Result<Body> {
        Ok(self
            .storage
            .get_exported_document_as_stream(id)
            .await?
            .into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<dto::Document> {
        let deleted = Document::delete(&self.pool, id).await?.into();
        self.storage.delete_document(id).await?;
        Ok(deleted)
    }

    pub async fn schedule_indexing(&self, document_id: Uuid) -> Result<dto::Indexing> {
        Ok(Indexing::schedule(&self.pool, document_id).await?.into())
    }
}
