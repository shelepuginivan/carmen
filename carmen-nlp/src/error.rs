#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("environment variable {0} is invalid: {1}")]
    InvalidEnvVar(&'static str, String),
    #[error("environment variable {0} (element {1}) is invalid: {1}")]
    InvalidEnvVarVec(&'static str, usize, String),
    #[error("{0}")]
    Fastembed(#[from] fastembed::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
