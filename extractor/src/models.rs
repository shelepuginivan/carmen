use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Document {
    #[serde(rename = "document_id")]
    pub id: String,
    pub object_key: String,
}

#[derive(Serialize)]
pub struct Chunk {
    #[serde(rename = "document_id")]
    pub id: String,
    pub text: String,
}
