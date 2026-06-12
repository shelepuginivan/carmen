use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

use crate::service;

pub struct Error {
    pub status: StatusCode,
    pub detail: String,
}

/// API error with detail message
#[derive(Serialize, ToSchema)]
pub struct ErrorWithDetail {
    pub detail: String,
}

impl From<service::Error> for Error {
    fn from(err: service::Error) -> Self {
        let status = match err {
            service::Error::Conflict(_) => StatusCode::CONFLICT,
            service::Error::NotFound(_) | service::Error::EntityNotFound => StatusCode::NOT_FOUND,

            service::Error::DatabaseError | service::Error::Anyhow(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        Self {
            status,
            detail: err.to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let detail = ErrorWithDetail {
            detail: self.detail,
        };
        (self.status, Json(detail)).into_response()
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
