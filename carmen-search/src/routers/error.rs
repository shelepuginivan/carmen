use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::service;

pub struct Error {
    pub status: StatusCode,
    pub detail: String,
}

impl From<service::Error> for Error {
    fn from(err: service::Error) -> Self {
        let status = match err {
            service::Error::Conflict(_) => StatusCode::CONFLICT,
            service::Error::NotFound(_) => StatusCode::NOT_FOUND,

            service::Error::Sqlx(_) | service::Error::Anyhow(_) => {
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
        #[derive(Serialize)]
        struct Detail {
            detail: String,
        }

        (
            self.status,
            Json(Detail {
                detail: self.detail,
            }),
        )
            .into_response()
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
