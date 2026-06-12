use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::{Json, Router};
use sqlx::PgPool;

use crate::service::collections;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/tasks/retry", patch(tasks_retry))
        .route("/tasks/retry-failed", patch(tasks_retry_failed))
}

/// Retry a specific collection indexing task
#[utoipa::path(
    patch,
    path = "/api/v1/collections/tasks/retry",
    request_body = collections::CollectionTaskRetryIn,
    responses(
        (
            status = 202,
            description = "Task rescheduled successfully",
            body = collections::CollectionTaskMetaOut,
        ),
        (
            status = 404,
            description = "Task does not exist",
            body = ErrorWithDetail,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn tasks_retry(
    db: State<PgPool>,
    Json(task_retry): Json<collections::CollectionTaskRetryIn>,
) -> Result<impl IntoResponse> {
    let retried_tasks = collections::retry(&db, task_retry).await?;

    Ok((StatusCode::ACCEPTED, Json(retried_tasks)))
}

/// Retry failed collection indexing tasks
#[utoipa::path(
    patch,
    path = "/api/v1/collections/tasks/retry-failed",
    responses(
        (
            status = 202,
            description = "Tasks rescheduled successfully",
            body = Vec<collections::CollectionTaskMetaOut>,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn tasks_retry_failed(db: State<PgPool>) -> Result<impl IntoResponse> {
    let retried_tasks = collections::retry_failed_tasks(&db).await?;

    Ok((StatusCode::ACCEPTED, Json(retried_tasks)))
}
