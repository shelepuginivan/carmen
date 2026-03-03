use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Document {
    document_id: String,
    object_key: String,
}

#[derive(Serialize)]
pub struct Chunk {
    document_id: String,
    text: String,
}
