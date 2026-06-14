use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::service::collections::{self};

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", post(create))
        .route("/", get(get_all))
        .route("/{id}", get(get_by_id))
        .route("/{id}", patch(update))
        .route("/{id}", delete(delete_collection))
        .route("/{id}/extractions", get(get_extractions))
        .route("/{id}/schedule", post(schedule_extraction))
}

/// Create new collection
#[utoipa::path(
    post,
    path = "/api/v1/collections",
    request_body = collections::CollectionIn,
    responses(
        (
            status = 201,
            description = "Collection created successfully",
            body = collections::CollectionOut,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn create(
    db: State<PgPool>,
    Json(collection_in): Json<collections::CollectionIn>,
) -> Result<impl IntoResponse> {
    let collection = collections::create_collection(&db, collection_in).await?;
    Ok((StatusCode::CREATED, Json(collection)))
}

/// Get all collections
#[utoipa::path(
    get,
    path = "/api/v1/collections",
    responses(
        (
            status = 200,
            description = "Collections",
            body = collections::CollectionOut,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_all(db: State<PgPool>) -> Result<impl IntoResponse> {
    let collections = collections::get_all_collections(&db).await?;
    Ok((StatusCode::OK, Json(collections)))
}

/// Get collection by id
#[utoipa::path(
    get,
    path = "/api/v1/collections/{id}",
    params(
        ("id" = Uuid, Path, description = "Collection ID")
    ),
    responses(
        (
            status = 200,
            description = "The requested collection",
            body = collections::CollectionOut,
        ),
        (
            status = 404,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_by_id(db: State<PgPool>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    let collections = collections::get_collection(&db, id).await?;
    Ok((StatusCode::OK, Json(collections)))
}

/// Update collection
#[utoipa::path(
    patch,
    path = "/api/v1/collections/{id}",
    request_body = collections::CollectionUpdate,
    responses(
        (
            status = 200,
            description = "Collection updated successfully",
            body = collections::CollectionOut,
        ),
        (
            status = 404,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn update(
    db: State<PgPool>,
    Json(collection_update): Json<collections::CollectionUpdate>,
) -> Result<impl IntoResponse> {
    let collection = collections::update_collection(&db, collection_update).await?;
    Ok((StatusCode::OK, Json(collection)))
}

/// Delete collection
#[utoipa::path(
    delete,
    path = "/api/v1/collections/{id}",
    params(
        ("id" = Uuid, Path, description = "Collection ID")
    ),
    responses(
        (
            status = 200,
            description = "Collection deleted",
            body = collections::CollectionExtractionOut,
        ),
        (
            status = 404,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn delete_collection(
    db: State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let extraction = collections::delete_collection(&db, id).await?;
    Ok((StatusCode::OK, Json(extraction)))
}

/// Get collection extractions
#[utoipa::path(
    get,
    path = "/api/v1/collections/{id}/extractions",
    params(
        ("id" = Uuid, Path, description = "Collection ID")
    ),
    responses(
        (
            status = 200,
            description = "Extractions of the collection",
            body = Vec<collections::CollectionExtractionOut>,
        ),
        (
            status = 404,
            description = "No extractions found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_extractions(db: State<PgPool>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    let extractions = collections::get_extractions(&db, id).await?;
    Ok((StatusCode::OK, Json(extractions)))
}

/// Schedule a new extraction of this collection
#[utoipa::path(
    post,
    path = "/api/v1/collections/{id}/schedule",
    params(
        ("id" = Uuid, Path, description = "Collection ID")
    ),
    responses(
        (
            status = 202,
            description = "Scheduled extraction",
            body = collections::CollectionExtractionOut,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn schedule_extraction(
    db: State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let extraction = collections::schedule_extraction(&db, id).await?;
    Ok((StatusCode::ACCEPTED, Json(extraction)))
}
