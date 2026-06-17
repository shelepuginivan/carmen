use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::app::AppState;
use crate::service::search;
use crate::service::search::dto::SearchParameters;

use super::error::{ErrorWithDetail, Result};

pub fn router() -> Router<AppState> {
    Router::new().route("/semantic", get(semantic))
}

/// Semantic search
#[utoipa::path(
    get,
    path = "/api/v1/search/semantic",
    params(SearchParameters),
    responses(
        (
            status = 200,
            description = "Search results",
            body = Vec<search::dto::Chunk>,
        ),
        (
            status = 500,
            description = "Internal server error occurred",
            body = ErrorWithDetail,
        )
    ),
)]
pub async fn semantic(
    state: State<AppState>,
    Query(params): Query<SearchParameters>,
) -> Result<impl IntoResponse> {
    let results = state.search.semantic(params).await?;
    Ok((StatusCode::OK, Json(results)))
}
