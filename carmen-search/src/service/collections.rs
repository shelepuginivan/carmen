use carmen_db::collections::{Collection, CollectionExtraction, CollectionExtractionType};
use carmen_db::documents::Document;
use carmen_db::types::Status;
use carmen_s3::Storage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use super::error::Result;

#[derive(Deserialize, ToSchema)]
#[schema(title = "CollectionIn")]
pub struct CollectionIn {
    name: String,
    description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[schema(title = "CollectionUpdate")]
pub struct CollectionUpdate {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[schema(title = "Collection")]
pub struct CollectionOut {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl From<Collection> for CollectionOut {
    fn from(value: Collection) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CollectionExtractionIn {
    collection_id: Uuid,
    source: String,
    source_type: String,
    extraction_type: Option<CollectionExtractionTypeOut>,
}

#[derive(Serialize, ToSchema)]
pub enum StatusOut {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl From<Status> for StatusOut {
    fn from(value: Status) -> Self {
        match value {
            Status::Pending => Self::Pending,
            Status::InProgress => Self::InProgress,
            Status::Completed => Self::Completed,
            Status::Failed => Self::Failed,
            Status::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum CollectionExtractionTypeOut {
    Merge,
    Override,
}

impl From<CollectionExtractionType> for CollectionExtractionTypeOut {
    fn from(value: CollectionExtractionType) -> Self {
        match value {
            CollectionExtractionType::Merge => Self::Merge,
            CollectionExtractionType::Override => Self::Override,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[schema(title = "CollectionExtraction")]
pub struct CollectionExtractionOut {
    id: Uuid,
    collection_id: Uuid,
    status: StatusOut,
    created_at: DateTime<Utc>,
    source: String,
    source_type: String,
    extraction_type: CollectionExtractionTypeOut,
}

impl From<CollectionExtraction> for CollectionExtractionOut {
    fn from(value: CollectionExtraction) -> Self {
        Self {
            id: value.id,
            collection_id: value.collection_id,
            status: value.status.into(),
            created_at: value.created_at,
            source: value.source,
            source_type: value.source_type,
            extraction_type: value.extraction_type.into(),
        }
    }
}

impl From<CollectionExtractionTypeOut> for CollectionExtractionType {
    fn from(val: CollectionExtractionTypeOut) -> Self {
        match val {
            CollectionExtractionTypeOut::Merge => CollectionExtractionType::Merge,
            CollectionExtractionTypeOut::Override => CollectionExtractionType::Override,
        }
    }
}

pub async fn create_collection(db: &PgPool, collection_in: CollectionIn) -> Result<CollectionOut> {
    Ok(Collection::insert(
        db,
        collection_in.name.as_ref(),
        collection_in.description.as_deref(),
    )
    .await?
    .into())
}

pub async fn get_all_collections(db: &PgPool) -> Result<Vec<CollectionOut>> {
    Ok(Collection::get_all(db)
        .await?
        .into_iter()
        .map(CollectionOut::from)
        .collect())
}

pub async fn get_collection(db: &PgPool, id: Uuid) -> Result<CollectionOut> {
    Ok(Collection::get(db, id).await?.into())
}

pub async fn get_extractions(db: &PgPool, id: Uuid) -> Result<Vec<CollectionExtractionOut>> {
    Ok(Collection::get_extractions(db, id)
        .await?
        .into_iter()
        .map(CollectionExtractionOut::from)
        .collect())
}

pub async fn schedule_extraction(
    db: &PgPool,
    extraction: CollectionExtractionIn,
) -> Result<CollectionExtractionOut> {
    Ok(Collection::schedule_extraction(
        db,
        extraction.collection_id,
        &extraction.source,
        &extraction.source_type,
        extraction
            .extraction_type
            .map(CollectionExtractionTypeOut::into),
    )
    .await?
    .into())
}

pub async fn update_collection(db: &PgPool, update: CollectionUpdate) -> Result<CollectionOut> {
    Ok(Collection::update(
        db,
        update.id,
        update.name.as_deref(),
        update.description.as_deref(),
    )
    .await?
    .into())
}

pub async fn delete_collection(db: &PgPool, storage: &Storage, id: Uuid) -> Result<CollectionOut> {
    let document_ids: Vec<Uuid> = Document::get_for_collection(db, id)
        .await?
        .into_iter()
        .map(|doc| doc.id)
        .collect();

    storage.delete_documents(&document_ids).await?;

    Ok(Collection::delete(db, id).await?.into())
}
