#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("environment variable {0} is invalid: {1}")]
    InvalidEnvVar(&'static str, String),
    #[error("{0}")]
    Fastembed(#[from] fastembed::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
