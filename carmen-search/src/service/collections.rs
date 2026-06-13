use carmen_db::collections::{Collection, CollectionExtraction, CollectionExtractionStatus};
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

#[derive(Serialize, ToSchema)]
#[schema(title = "Collection")]
pub struct CollectionOut {
    id: Uuid,
    name: String,
    description: Option<String>,
    url: Option<String>,
    source: String,
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
pub enum CollectionBuildStatusOut {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl From<CollectionExtractionStatus> for CollectionBuildStatusOut {
    fn from(value: CollectionExtractionStatus) -> Self {
        match value {
            CollectionExtractionStatus::Pending => Self::Pending,
            CollectionExtractionStatus::InProgress => Self::InProgress,
            CollectionExtractionStatus::Completed => Self::Completed,
            CollectionExtractionStatus::Failed => Self::Failed,
            CollectionExtractionStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[schema(title = "CollectionExtraction")]
pub struct CollectionExtractionOut {
    id: Uuid,
    collection_id: Uuid,
    status: CollectionBuildStatusOut,
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
