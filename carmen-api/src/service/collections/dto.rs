use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateCollection {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCollection {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl From<carmen_db::collections::Collection> for Collection {
    fn from(value: carmen_db::collections::Collection) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectionExtractionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl From<carmen_db::collections::CollectionExtractionStatus> for CollectionExtractionStatus {
    fn from(value: carmen_db::collections::CollectionExtractionStatus) -> Self {
        match value {
            carmen_db::collections::CollectionExtractionStatus::Pending => Self::Pending,
            carmen_db::collections::CollectionExtractionStatus::InProgress => Self::InProgress,
            carmen_db::collections::CollectionExtractionStatus::Completed => Self::Completed,
            carmen_db::collections::CollectionExtractionStatus::Failed => Self::Failed,
            carmen_db::collections::CollectionExtractionStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CollectionExtractionType {
    Merge,
    Override,
}

impl From<carmen_db::collections::CollectionExtractionType> for CollectionExtractionType {
    fn from(value: carmen_db::collections::CollectionExtractionType) -> Self {
        match value {
            carmen_db::collections::CollectionExtractionType::Merge => Self::Merge,
            carmen_db::collections::CollectionExtractionType::Override => Self::Override,
        }
    }
}

impl From<CollectionExtractionType> for carmen_db::collections::CollectionExtractionType {
    fn from(val: CollectionExtractionType) -> Self {
        match val {
            CollectionExtractionType::Merge => Self::Merge,
            CollectionExtractionType::Override => Self::Override,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleCollectionExtraction {
    pub collection_id: Uuid,
    pub source: String,
    pub source_type: String,
    pub parameters: serde_json::Value,
    pub extraction_type: CollectionExtractionType,
}

#[derive(Serialize, ToSchema)]
pub struct CollectionExtraction {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: CollectionExtractionStatus,
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub source_type: String,
    pub extraction_type: CollectionExtractionType,
}

impl From<carmen_db::collections::CollectionExtraction> for CollectionExtraction {
    fn from(value: carmen_db::collections::CollectionExtraction) -> Self {
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

#[derive(Serialize, ToSchema)]
pub struct CancellationResult {
    pub cancelled: bool,
}
