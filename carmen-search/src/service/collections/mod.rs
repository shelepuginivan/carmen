use carmen_db::collections::Collection;
use carmen_db::documents::Document;
use carmen_s3::Storage;
use sqlx::PgPool;
use uuid::Uuid;

use super::error::Result;

pub mod dto;

pub async fn create_collection(
    db: &PgPool,
    dto::CreateCollection { name, description }: dto::CreateCollection,
) -> Result<dto::Collection> {
    Ok(
        Collection::insert(db, name.as_ref(), description.as_deref())
            .await?
            .into(),
    )
}

pub async fn get_all_collections(db: &PgPool) -> Result<Vec<dto::Collection>> {
    Ok(Collection::get_all(db)
        .await?
        .into_iter()
        .map(dto::Collection::from)
        .collect())
}

pub async fn get_collection(db: &PgPool, id: Uuid) -> Result<dto::Collection> {
    Ok(Collection::get(db, id).await?.into())
}

pub async fn get_extractions(db: &PgPool, id: Uuid) -> Result<Vec<dto::CollectionExtraction>> {
    Ok(Collection::get_extractions(db, id)
        .await?
        .into_iter()
        .map(dto::CollectionExtraction::from)
        .collect())
}

pub async fn schedule_extraction(
    db: &PgPool,
    dto::ScheduleCollectionExtraction {
        collection_id,
        source,
        source_type,
        extraction_type,
    }: dto::ScheduleCollectionExtraction,
) -> Result<dto::CollectionExtraction> {
    Ok(Collection::schedule_extraction(
        db,
        collection_id,
        &source,
        &source_type,
        extraction_type.into(),
    )
    .await?
    .into())
}

pub async fn update_collection(
    db: &PgPool,
    dto::UpdateCollection {
        id,
        name,
        description,
    }: dto::UpdateCollection,
) -> Result<dto::Collection> {
    Ok(
        Collection::update(db, id, name.as_deref(), description.as_deref())
            .await?
            .into(),
    )
}

pub async fn delete_collection(
    db: &PgPool,
    storage: &Storage,
    id: Uuid,
) -> Result<dto::Collection> {
    let document_ids: Vec<Uuid> = Document::get_for_collection(db, id)
        .await?
        .into_iter()
        .map(|doc| doc.id)
        .collect();

    storage.delete_documents(&document_ids).await?;

    Ok(Collection::delete(db, id).await?.into())
}
