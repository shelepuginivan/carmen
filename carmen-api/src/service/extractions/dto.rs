use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl From<carmen_db::extractions::ExtractionStatus> for ExtractionStatus {
    fn from(value: carmen_db::extractions::ExtractionStatus) -> Self {
        match value {
            carmen_db::extractions::ExtractionStatus::Pending => Self::Pending,
            carmen_db::extractions::ExtractionStatus::InProgress => Self::InProgress,
            carmen_db::extractions::ExtractionStatus::Completed => Self::Completed,
            carmen_db::extractions::ExtractionStatus::Failed => Self::Failed,
            carmen_db::extractions::ExtractionStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionType {
    Merge,
    Override,
}

impl From<carmen_db::extractions::ExtractionType> for ExtractionType {
    fn from(value: carmen_db::extractions::ExtractionType) -> Self {
        match value {
            carmen_db::extractions::ExtractionType::Merge => Self::Merge,
            carmen_db::extractions::ExtractionType::Override => Self::Override,
        }
    }
}

impl From<ExtractionType> for carmen_db::extractions::ExtractionType {
    fn from(val: ExtractionType) -> Self {
        match val {
            ExtractionType::Merge => Self::Merge,
            ExtractionType::Override => Self::Override,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleExtraction {
    pub collection_id: Uuid,
    pub source: String,
    pub source_type: String,
    pub parameters: serde_json::Value,
    pub extraction_type: ExtractionType,
}

#[derive(Serialize, ToSchema)]
pub struct Extraction {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub status: ExtractionStatus,
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub source_type: String,
    pub extraction_type: ExtractionType,
    pub parameters: serde_json::Value,
}

impl From<carmen_db::extractions::Extraction> for Extraction {
    fn from(value: carmen_db::extractions::Extraction) -> Self {
        Self {
            id: value.id,
            collection_id: value.collection_id,
            status: value.status.into(),
            created_at: value.created_at,
            source: value.source,
            source_type: value.source_type,
            extraction_type: value.extraction_type.into(),
            parameters: value.parameters,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct CancellationResult {
    pub cancelled: bool,
}
