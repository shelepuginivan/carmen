use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::app::AppState;
use crate::service::extractions;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_extraction))
        .route("/{id}", delete(delete_extraction))
        .route("/{id}/cancel", post(cancel_extraction))
        .route("/{id}/replay", post(replay_extraction))
}

/// Get extraction by id
#[utoipa::path(
    get,
    path = "/api/v1/extractions/{id}",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = 200,
            description = "Extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn get_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let deleted = state.extractions.get_by_id(id).await?;
    Ok((StatusCode::OK, Json(deleted)))
}

/// Delete an extraction
#[utoipa::path(
    delete,
    path = "/api/v1/extractions/{id}",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = 200,
            description = "Deleted extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn delete_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let deleted = state.extractions.delete(id).await?;
    Ok((StatusCode::OK, Json(deleted)))
}

/// Cancel an extraction
#[utoipa::path(
    post,
    path = "/api/v1/extractions/{id}/cancel",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = 200,
            description = "Cancellation result",
            body = extractions::dto::CancellationResult,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn cancel_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let result = state.extractions.cancel(id).await?;
    Ok((StatusCode::OK, Json(result)))
}

/// Replay an extraction
#[utoipa::path(
    post,
    path = "/api/v1/extractions/{id}/replay",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = 202,
            description = "Scheduled extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn replay_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let replay = state.extractions.replay(id).await?;
    Ok((StatusCode::ACCEPTED, Json(replay)))
}
