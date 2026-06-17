use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchParameters {
    #[serde(rename = "q")]
    pub query: String,
    pub collection: Uuid,
    pub limit: Option<u16>,
}

#[derive(Serialize, ToSchema)]
pub struct Chunk {
    pub id: Uuid,
    pub document_id: Uuid,
    pub text: String,
    pub language: String,
}

impl From<carmen_db::chunks::Chunk> for Chunk {
    fn from(value: carmen_db::chunks::Chunk) -> Self {
        Self {
            id: value.id,
            document_id: value.document_id,
            text: value.text,
            language: value.language,
        }
    }
}
