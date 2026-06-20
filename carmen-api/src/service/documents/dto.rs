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
