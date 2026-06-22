use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::app::AppState;
use crate::service::documents;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_by_id))
        .route("/{id}/raw", get(raw_document))
        .route("/{id}/exported", get(exported_document))
        .route("/{id}", delete(delete_document))
        .route("/{id}/schedule", post(schedule_indexing))
}

/// Get document by id
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (
            status = 200,
            description = "The requested document",
            body = documents::dto::Document,
        ),
        (
            status = 404,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_by_id(state: State<AppState>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    let document = state.documents.get_one(id).await?;
    Ok((StatusCode::OK, Json(document)))
}

/// Raw document
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/raw",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (
            status = 200,
            description = "Raw document",
        ),
        (
            status = 404,
            description = "Document not found",
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn raw_document(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let body = state.documents.get_raw_stream(id).await?;
    Ok((StatusCode::OK, body))
}

/// Exported document
#[utoipa::path(
    get,
    path = "/api/v1/documents/{id}/exported",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (
            status = 200,
            description = "Exported document",
            content_type = "text/markdown",
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn exported_document(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let body = state.documents.get_exported_stream(id).await?;
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "text/markdown".parse().unwrap());

    Ok((StatusCode::OK, headers, body))
}

/// Delete document
#[utoipa::path(
    delete,
    path = "/api/v1/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (
            status = 200,
            description = "Document deleted",
            body = documents::dto::Document,
        ),
        (
            status = 404,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn delete_document(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let document = state.documents.delete(id).await?;
    Ok((StatusCode::OK, Json(document)))
}

/// Schedule document indexing
#[utoipa::path(
    post,
    path = "/api/v1/documents/{id}/schedule",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (
            status = 202,
            description = "Indexing scheduled",
            body = documents::dto::DocumentIndexing,
        ),
        (
            status = 404,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn schedule_indexing(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let indexing = state.documents.schedule_indexing(id).await?;
    Ok((StatusCode::ACCEPTED, Json(indexing)))
}
