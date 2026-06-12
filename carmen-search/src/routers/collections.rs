use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::{Json, Router};
use sqlx::PgPool;

use crate::service::collections;

use super::error::Result;

pub fn router() -> Router<PgPool> {
    Router::new().route("/tasks/retry-failed", patch(tasks_retry_failed))
}

async fn tasks_retry_failed(db: State<PgPool>) -> Result<impl IntoResponse> {
    let retried_tasks = collections::retry_failed_tasks(&db).await?;

    Ok((StatusCode::ACCEPTED, Json(retried_tasks)))
}
