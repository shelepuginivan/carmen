use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use utoipa::OpenApi;

use crate::app::AppState;
use crate::service::search;

use super::error::{ErrorWithDetail, Result};

#[derive(OpenApi)]
#[openapi(paths(full_text, semantic, hybrid))]
pub struct ApiDoc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/semantic", get(semantic))
        .route("/fulltext", get(full_text))
        .route("/hybrid", get(hybrid))
}

/// Full text search
#[utoipa::path(
    get,
    path = "/fulltext",
    params(search::dto::SearchParameters),
    responses(
        (
            status = OK,
            description = "Search results",
            body = Vec<search::dto::Chunk>,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn full_text(
    state: State<AppState>,
    Query(params): Query<search::dto::SearchParameters>,
) -> Result<impl IntoResponse> {
    let results = state.search.full_text(params).await?;
    Ok((StatusCode::OK, Json(results)))
}

/// Semantic search
#[utoipa::path(
    get,
    path = "/semantic",
    params(search::dto::SearchParameters),
    responses(
        (
            status = OK,
            description = "Search results",
            body = Vec<search::dto::Chunk>,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn semantic(
    state: State<AppState>,
    Query(params): Query<search::dto::SearchParameters>,
) -> Result<impl IntoResponse> {
    let results = state.search.semantic(params).await?;
    Ok((StatusCode::OK, Json(results)))
}

/// Hybrid search
#[utoipa::path(
    get,
    path = "/hybrid",
    params(search::dto::SearchParameters),
    responses(
        (
            status = OK,
            description = "Search results",
            body = Vec<search::dto::Chunk>,
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
async fn hybrid(
    state: State<AppState>,
    Query(params): Query<search::dto::SearchParameters>,
) -> Result<impl IntoResponse> {
    let results = state.search.hybrid(params).await?;
    Ok((StatusCode::OK, Json(results)))
}
