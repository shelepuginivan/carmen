use std::sync::Arc;

use axum::body::Body;
use carmen_db::documents::Document;
use carmen_s3::Storage;
use sqlx::PgPool;
use uuid::Uuid;

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

    pub async fn get_from_collection(&self, collection_id: Uuid) -> Result<Vec<dto::Document>> {
        Ok(Document::get_for_collection(&self.pool, collection_id)
            .await?
            .into_iter()
            .map(dto::Document::from)
            .collect())
    }

    pub async fn get_raw_stream(&self, id: Uuid) -> Result<Body> {
        let stream = self.storage.get_raw_document_as_stream(id).await?;

        Ok(Body::from_stream(stream.bytes))
    }

    pub async fn get_exported_stream(&self, id: Uuid) -> Result<Body> {
        let stream = self.storage.get_exported_document_as_stream(id).await?;

        Ok(Body::from_stream(stream.bytes))
    }
}
