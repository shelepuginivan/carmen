use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::{Json, Router};
use sqlx::PgPool;

use crate::service;

pub fn router() -> Router<PgPool> {
    Router::new().route("/tasks/retry-failed", patch(tasks_retry_failed))
}

async fn tasks_retry_failed(db: State<PgPool>) -> impl IntoResponse {
    // FIXME: handle errors properly
    let retried_tasks = service::collections::retry_failed_tasks(&db).await.unwrap();

    (StatusCode::ACCEPTED, Json(retried_tasks))
}
