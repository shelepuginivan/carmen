#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    NotFound(String),

    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
