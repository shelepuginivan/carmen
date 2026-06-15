use s3::error::S3Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entity not found")]
    EntityNotFound,

    #[error("an internal database error occurred")]
    Database,

    #[error("an internal storage error occurred")]
    S3Error(#[from] S3Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::EntityNotFound,
            _ => Self::Database,
        }
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
