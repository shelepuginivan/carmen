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
pub enum IndexingStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl From<carmen_db::indexing::IndexingStatus> for IndexingStatus {
    fn from(value: carmen_db::indexing::IndexingStatus) -> Self {
        match value {
            carmen_db::indexing::IndexingStatus::Pending => Self::Pending,
            carmen_db::indexing::IndexingStatus::InProgress => Self::InProgress,
            carmen_db::indexing::IndexingStatus::Completed => Self::Completed,
            carmen_db::indexing::IndexingStatus::Failed => Self::Failed,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Indexing {
    pub id: Uuid,
    pub document_id: Uuid,
    pub status: IndexingStatus,
    pub created_at: DateTime<Utc>,
}

impl From<carmen_db::indexing::Indexing> for Indexing {
    fn from(value: carmen_db::indexing::Indexing) -> Self {
        Self {
            id: value.id,
            document_id: value.document_id,
            status: value.status.into(),
            created_at: value.created_at,
        }
    }
}
