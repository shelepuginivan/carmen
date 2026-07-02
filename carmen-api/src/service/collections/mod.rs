use std::sync::Arc;

use carmen_db::collections::{Collection, CollectionExtraction};
use carmen_db::documents::Document;
use carmen_s3::Storage;
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

    pub async fn create(
        &self,
        dto::CreateCollection { name, description }: dto::CreateCollection,
    ) -> Result<dto::Collection> {
        Ok(
            Collection::insert(&self.pool, name.as_ref(), description.as_deref())
                .await?
                .into(),
        )
    }

    pub async fn get_all(&self) -> Result<Vec<dto::Collection>> {
        Ok(Collection::get_all(&self.pool)
            .await?
            .into_iter()
            .map(dto::Collection::from)
            .collect())
    }

    pub async fn get_one(&self, id: Uuid) -> Result<dto::Collection> {
        Ok(Collection::get(&self.pool, id).await?.into())
    }

    pub async fn update(
        &self,
        dto::UpdateCollection {
            id,
            name,
            description,
        }: dto::UpdateCollection,
    ) -> Result<dto::Collection> {
        Ok(
            Collection::update(&self.pool, id, name.as_deref(), description.as_deref())
                .await?
                .into(),
        )
    }

    pub async fn delete(&self, id: Uuid) -> Result<dto::Collection> {
        let document_ids: Vec<Uuid> = Document::get_for_collection(&self.pool, id)
            .await?
            .into_iter()
            .map(|doc| doc.id)
            .collect();

        self.storage.delete_documents(&document_ids).await?;

        Ok(Collection::delete(&self.pool, id).await?.into())
    }

    pub async fn get_extractions(&self, id: Uuid) -> Result<Vec<dto::CollectionExtraction>> {
        Ok(Collection::get_extractions(&self.pool, id)
            .await?
            .into_iter()
            .map(dto::CollectionExtraction::from)
            .collect())
    }

    pub async fn schedule_extraction(
        &self,
        dto::ScheduleCollectionExtraction {
            collection_id,
            source,
            source_type,
            parameters,
            extraction_type,
        }: dto::ScheduleCollectionExtraction,
    ) -> Result<dto::CollectionExtraction> {
        Ok(Collection::schedule_extraction(
            &self.pool,
            collection_id,
            &source,
            &source_type,
            extraction_type.into(),
            &parameters,
        )
        .await?
        .into())
    }

    pub async fn cancel_extraction(&self, id: Uuid) -> Result<dto::CancellationResult> {
        let cancelled = CollectionExtraction::cancel(&self.pool, id).await?;
        Ok(dto::CancellationResult { cancelled })
    }
}
