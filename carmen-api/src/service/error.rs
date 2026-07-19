#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entity not found")]
    EntityNotFound,

    #[error("object not found")]
    ObjectNotFound,

    #[error("an internal database error occurred")]
    Database,

    #[error("an internal storage error occurred")]
    Storage,

    #[error("an internal server error occurred")]
    Nlp(#[from] carmen_nlp::Error),

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

impl From<carmen_storage::Error> for Error {
    fn from(value: carmen_storage::Error) -> Self {
        match value {
            carmen_storage::Error::NotFound => Self::ObjectNotFound,
            _ => Self::Storage,
        }
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
