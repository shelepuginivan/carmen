use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::app::AppState;
use crate::service::extractions;

use super::error::{ErrorWithDetail, Result};

#[derive(OpenApi)]
#[openapi(paths(
    get_extraction,
    delete_extraction,
    cancel_extraction,
    replay_extraction,
))]
pub struct ApiDoc;

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
    path = "/{id}",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = OK,
            description = "Extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = NOT_FOUND,
            description = "Extraction not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn get_extraction(state: State<AppState>, Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    let deleted = state.extractions.get(id).await?;
    Ok((StatusCode::OK, Json(deleted)))
}

/// Delete an extraction
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = OK,
            description = "Deleted extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = NOT_FOUND,
            description = "Extraction not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn delete_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let deleted = state.extractions.delete(id).await?;
    Ok((StatusCode::OK, Json(deleted)))
}

/// Cancel an extraction
#[utoipa::path(
    post,
    path = "/{id}/cancel",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = OK,
            description = "Cancellation result",
            body = extractions::dto::CancellationResult,
        ),
        (
            status = NOT_FOUND,
            description = "Extraction not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn cancel_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let result = state.extractions.cancel(id).await?;
    Ok((StatusCode::OK, Json(result)))
}

/// Replay an extraction
#[utoipa::path(
    post,
    path = "/{id}/replay",
    params(
        ("id" = Uuid, Path, description = "Extraction ID")
    ),
    responses(
        (
            status = ACCEPTED,
            description = "Scheduled extraction",
            body = extractions::dto::Extraction,
        ),
        (
            status = NOT_FOUND,
            description = "Extraction not found",
            body = ErrorWithDetail,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn replay_extraction(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let replay = state.extractions.replay(id).await?;
    Ok((StatusCode::ACCEPTED, Json(replay)))
}
