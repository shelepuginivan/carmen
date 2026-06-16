use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("environment variable CARMEN_POSTGRES_URL is required")]
    EnvURLMissing,
    #[error("could not connect to database after {0} retries")]
    MaxRetriesReached(u8),
}
