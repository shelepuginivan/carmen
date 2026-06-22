use std::str::Utf8Error;

use s3::{creds::error::CredentialsError, error::S3Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing environment variable: {0}")]
    Environment(&'static str),
    #[error("invalid configuration: {0}")]
    Configuration(String),
    #[error("object not found")]
    NotFound,
    #[error("{0}")]
    Runtime(String),
    #[error("object is not a valid UTF-8 string: {0}")]
    UTF8(#[from] Utf8Error),
    #[error("{0}")]
    IO(#[from] tokio::io::Error),
}

impl From<S3Error> for Error {
    fn from(value: S3Error) -> Self {
        match value {
            S3Error::Region(err) => Self::Configuration(err.to_string()),
            S3Error::Credentials(err) => Self::Configuration(err.to_string()),
            S3Error::HttpFailWithBody(status, _) => match status {
                404 => Self::NotFound,
                _ => Self::Runtime("unknown http error".to_string()),
            },

            other => Self::Runtime(other.to_string()),
        }
    }
}

impl From<CredentialsError> for Error {
    fn from(value: CredentialsError) -> Self {
        Self::Configuration(value.to_string())
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
