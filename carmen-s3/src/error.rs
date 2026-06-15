use s3::error::S3Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("missing environment variable: {0}")]
    Environment(&'static str),
    #[error("invalid configuration: {0}")]
    Configuration(String),
    #[error("{0}")]
    Runtime(String),
}

impl From<S3Error> for StorageError {
    fn from(value: S3Error) -> Self {
        match value {
            S3Error::Region(err) => Self::Configuration(err.to_string()),
            S3Error::Credentials(err) => Self::Configuration(err.to_string()),

            other => Self::Runtime(other.to_string()),
        }
    }
}
