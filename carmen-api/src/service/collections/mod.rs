use std::sync::Arc;

use carmen_db::collections::Collection;
use carmen_db::documents::Document;
use carmen_storage::Storage;
use sqlx::PgPool;
use uuid::Uuid;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct CollectionService {
    pool: PgPool,
    storage: Arc<Storage>,
}

impl CollectionService {
    pub fn new(pool: PgPool, storage: Arc<Storage>) -> Self {
        Self { pool, storage }
    }

    pub async fn create(&self, v: dto::CreateCollection) -> Result<dto::Collection> {
        Ok(
            Collection::insert(&self.pool, v.name.as_ref(), v.description.as_deref())
                .await?
                .into(),
        )
    }

    pub async fn get(&self, id: Uuid) -> Result<dto::Collection> {
        Ok(Collection::get(&self.pool, id).await?.into())
    }

    pub async fn get_all(&self) -> Result<Vec<dto::Collection>> {
        Ok(Collection::get_all(&self.pool)
            .await?
            .into_iter()
            .map(dto::Collection::from)
            .collect())
    }

    pub async fn update(&self, id: Uuid, v: dto::UpdateCollection) -> Result<dto::Collection> {
        Ok(
            Collection::update(&self.pool, id, v.name.as_deref(), v.description.as_deref())
                .await?
                .into(),
        )
    }

    pub async fn delete(&self, id: Uuid) -> Result<dto::Collection> {
        let document_ids: Vec<Uuid> = Document::get_by_collection_id(&self.pool, id)
            .await?
            .into_iter()
            .map(|doc| doc.id)
            .collect();

        self.storage.delete_documents(&document_ids).await?;

        Ok(Collection::delete(&self.pool, id).await?.into())
    }
}
