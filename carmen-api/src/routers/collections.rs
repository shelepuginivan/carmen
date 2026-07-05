use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::app::AppState;
use crate::service::{collections, documents, extractions};

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_collection))
        .route("/", get(get_all_collections))
        .route("/{id}", get(get_collection))
        .route("/{id}", patch(update_collection))
        .route("/{id}", delete(delete_collection))
        .route("/{id}/documents", get(get_documents))
        .route("/{id}/extractions", get(get_extractions))
        .route("/{id}/extract", post(extract_collection))
}

/// Create new collection
#[utoipa::path(
    post,
    path = "/api/v1/collections",
    request_body = collections::dto::CreateCollection,
    responses(
        (
            status = CREATED,
            description = "Collection created successfully",
            body = collections::dto::Collection,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn create_collection(
    state: State<AppState>,
    Json(collection_in): Json<collections::dto::CreateCollection>,
) -> Result<impl IntoResponse> {
    let collection = state.collections.create(collection_in).await?;
    Ok((StatusCode::CREATED, Json(collection)))
}

/// Get all collections
#[utoipa::path(
    get,
    path = "/api/v1/collections",
    responses(
        (
            status = OK,
            description = "Collections",
            body = collections::dto::Collection,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_all_collections(state: State<AppState>) -> Result<impl IntoResponse> {
    let collections = state.collections.get_all().await?;
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
            status = OK,
            description = "The requested collection",
            body = collections::dto::Collection,
        ),
        (
            status = NOT_FOUND,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_collection(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let collection = state.collections.get(id).await?;
    Ok((StatusCode::OK, Json(collection)))
}

/// Update collection
#[utoipa::path(
    patch,
    path = "/api/v1/collections/{id}",
    request_body = collections::dto::UpdateCollection,
    responses(
        (
            status = OK,
            description = "Collection updated successfully",
            body = collections::dto::Collection,
        ),
        (
            status = NOT_FOUND,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn update_collection(
    state: State<AppState>,
    Json(collection_update): Json<collections::dto::UpdateCollection>,
) -> Result<impl IntoResponse> {
    let collection = state.collections.update(collection_update).await?;
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
            status = OK,
            description = "Collection deleted",
            body = collections::dto::Collection,
        ),
        (
            status = NOT_FOUND,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn delete_collection(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let collection = state.collections.delete(id).await?;
    Ok((StatusCode::OK, Json(collection)))
}

/// Get collection documents
#[utoipa::path(
    get,
    path = "/api/v1/collections/{id}/documents",
    params(
        ("id" = Uuid, Path, description = "Collection ID")
    ),
    responses(
        (
            status = OK,
            description = "Documents in this collection",
            body = Vec<documents::dto::Document>,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_documents(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let documents = state.documents.get_by_collection_id(id).await?;
    Ok((StatusCode::OK, Json(documents)))
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
            status = OK,
            description = "Extractions of the collection",
            body = Vec<extractions::dto::Extraction>,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_extractions(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let extractions = state.extractions.get_by_collection_id(id).await?;
    Ok((StatusCode::OK, Json(extractions)))
}

/// Schedule a new extraction of this collection
#[utoipa::path(
    post,
    path = "/api/v1/collections/{id}/extract",
    request_body = extractions::dto::ScheduleExtraction,
    responses(
        (
            status = ACCEPTED,
            description = "Scheduled extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = NOT_FOUND,
            description = "Collection not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn extract_collection(
    state: State<AppState>,
    Json(extraction): Json<extractions::dto::ScheduleExtraction>,
) -> Result<impl IntoResponse> {
    let extraction = state.extractions.schedule(extraction).await?;
    Ok((StatusCode::ACCEPTED, Json(extraction)))
}
