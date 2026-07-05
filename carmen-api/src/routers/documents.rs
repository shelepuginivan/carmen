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
        .route("/{id}", get(get_document))
        .route("/{id}/raw", get(stream_raw_document))
        .route("/{id}/exported", get(stream_exported_document))
        .route("/{id}", delete(delete_document))
        .route("/{id}/index", post(index_document))
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
            status = OK,
            description = "The requested document",
            body = documents::dto::Document,
        ),
        (
            status = NOT_FOUND,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_document(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let document = state.documents.get(id).await?;
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
            status = OK,
            description = "Raw document",
        ),
        (
            status = NOT_FOUND,
            description = "Document not found",
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn stream_raw_document(
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
            status = OK,
            description = "Exported document",
            content_type = "text/markdown",
        ),
        (
            status = NOT_FOUND,
            description = "Document not found",
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn stream_exported_document(
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
            status = OK,
            description = "Document deleted",
            body = documents::dto::Document,
        ),
        (
            status = NOT_FOUND,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
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
            status = ACCEPTED,
            description = "Indexing scheduled",
            body = documents::dto::Indexing,
        ),
        (
            status = NOT_FOUND,
            description = "Document not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn index_document(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let indexing = state.documents.schedule_indexing(id).await?;
    Ok((StatusCode::ACCEPTED, Json(indexing)))
}
