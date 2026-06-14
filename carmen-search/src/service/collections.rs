use carmen_db::collections::{Collection, CollectionExtraction};
use carmen_db::types::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use url::Url;
use utoipa::ToSchema;
use uuid::Uuid;

use super::error::Result;

#[derive(Deserialize, ToSchema)]
#[schema(title = "CollectionIn")]
pub struct CollectionIn {
    name: String,
    description: Option<String>,
    url: Option<Url>,
    source: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[schema(title = "CollectionUpdate")]
pub struct CollectionUpdate {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    url: Option<Url>,
    source: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[schema(title = "Collection")]
pub struct CollectionOut {
    id: Uuid,
    name: String,
    description: Option<String>,
    url: Option<String>,
    source: Option<String>,
}

impl From<Collection> for CollectionOut {
    fn from(value: Collection) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            url: value.url,
            source: value.source,
        }
    }
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

#[derive(Serialize, ToSchema)]
#[schema(title = "CollectionExtraction")]
pub struct CollectionExtractionOut {
    id: Uuid,
    collection_id: Uuid,
    status: StatusOut,
    created_at: DateTime<Utc>,
}

impl From<CollectionExtraction> for CollectionExtractionOut {
    fn from(value: CollectionExtraction) -> Self {
        Self {
            id: value.id,
            collection_id: value.collection_id,
            status: value.status.into(),
            created_at: value.created_at,
        }
    }
}

pub async fn create_collection(db: &PgPool, collection_in: CollectionIn) -> Result<CollectionOut> {
    Ok(Collection::insert(
        db,
        collection_in.name.as_ref(),
        collection_in.description.as_deref(),
        collection_in.url.as_ref().map(|u| u.as_str()),
        collection_in.source.as_deref(),
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

pub async fn schedule_extraction(db: &PgPool, id: Uuid) -> Result<CollectionExtractionOut> {
    Ok(Collection::schedule_extraction(db, id).await?.into())
}

pub async fn update_collection(db: &PgPool, update: CollectionUpdate) -> Result<CollectionOut> {
    Ok(Collection::update(
        db,
        update.id,
        update.name.as_deref(),
        update.description.as_deref(),
        update.url.as_ref().map(|u| u.as_str()),
        update.source.as_deref(),
    )
    .await?
    .into())
}

pub async fn delete_collection(db: &PgPool, id: Uuid) -> Result<CollectionOut> {
    // TODO: delete documents from s3
    Ok(Collection::delete(db, id).await?.into())
}
