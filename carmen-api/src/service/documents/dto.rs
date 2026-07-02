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
}

impl From<carmen_db::documents::DocumentIndexingStatus> for DocumentIndexingStatus {
    fn from(value: carmen_db::documents::DocumentIndexingStatus) -> Self {
        match value {
            carmen_db::documents::DocumentIndexingStatus::Pending => Self::Pending,
            carmen_db::documents::DocumentIndexingStatus::InProgress => Self::InProgress,
            carmen_db::documents::DocumentIndexingStatus::Completed => Self::Completed,
            carmen_db::documents::DocumentIndexingStatus::Failed => Self::Failed,
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
