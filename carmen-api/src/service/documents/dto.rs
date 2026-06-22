use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Document {
    pub id: Uuid,
    pub canonical_path: String,
}

impl From<carmen_db::documents::Document> for Document {
    fn from(value: carmen_db::documents::Document) -> Self {
        Self {
            id: value.id,
            canonical_path: value.canonical_path,
        }
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DocumentIndexingStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl From<carmen_db::types::Status> for DocumentIndexingStatus {
    fn from(value: carmen_db::types::Status) -> Self {
        match value {
            carmen_db::types::Status::Pending => Self::Pending,
            carmen_db::types::Status::InProgress => Self::InProgress,
            carmen_db::types::Status::Completed => Self::Completed,
            carmen_db::types::Status::Failed => Self::Failed,
            carmen_db::types::Status::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct DocumentIndexing {
    pub id: Uuid,
    pub document_id: Uuid,
    pub status: DocumentIndexingStatus,
    pub created_at: DateTime<Utc>,
}

impl From<carmen_db::documents::DocumentIndexing> for DocumentIndexing {
    fn from(value: carmen_db::documents::DocumentIndexing) -> Self {
        Self {
            id: value.id,
            document_id: value.document_id,
            status: value.status.into(),
            created_at: value.created_at,
        }
    }
}
