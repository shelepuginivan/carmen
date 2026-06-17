use carmen_db::documents::Document;
use sqlx::PgPool;
use uuid::Uuid;

use super::error::Result;

pub mod dto;

pub async fn get_documents_in_collection(
    pool: &PgPool,
    collection_id: Uuid,
) -> Result<Vec<dto::Document>> {
    Ok(Document::get_for_collection(pool, collection_id)
        .await?
        .into_iter()
        .map(dto::Document::from)
        .collect())
}
