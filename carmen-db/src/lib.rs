use std::env;
use std::time::Duration;

use log::{info, warn};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;

pub mod chunks;
pub mod collections;
pub mod documents;
pub mod error;
pub mod search;

use crate::error::ConnectionError;

const MAX_RETRIES: u8 = 5;
const BACKOFF_FACTOR: u32 = 2;
const DEFAULT_MAX_CONNECTIONS: u32 = 50;

pub async fn connect_from_env() -> Result<PgPool, ConnectionError> {
    let url = env::var("CARMEN_POSTGRES_URL").map_err(|_| ConnectionError::EnvURLMissing)?;

    let max_conn: u32 = env::var("CARMEN_POSTGRES_MAX_CONNECTIONS")
        .map(|v| v.parse().unwrap_or(DEFAULT_MAX_CONNECTIONS))
        .unwrap_or(DEFAULT_MAX_CONNECTIONS);

    let mut delay = Duration::from_secs(1);
    let mut attempt = 1;

    loop {
        if let Ok(pool) = PgPoolOptions::new()
            .max_connections(max_conn)
            .connect(&url)
            .await
        {
            info!("Database connection established");
            return Ok(pool);
        }

        if attempt >= MAX_RETRIES {
            break;
        }

        warn!(
            "Failed to connect to the database (attempt {attempt}/{MAX_RETRIES}). Retrying in {}s...",
            delay.as_secs()
        );

        sleep(delay).await;
        attempt += 1;
        delay *= BACKOFF_FACTOR;
    }

    Err(ConnectionError::MaxRetriesReached(MAX_RETRIES))
}
