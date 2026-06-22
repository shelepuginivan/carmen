use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use uuid::Uuid;

use crate::app::AppState;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}/raw", get(raw_document))
        .route("/{id}/exported", get(exported_document))
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
