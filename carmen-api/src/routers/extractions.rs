use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use uuid::Uuid;

use crate::app::AppState;
use crate::service::extractions;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new().route("/{id}/cancel", post(cancel_extraction))
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
