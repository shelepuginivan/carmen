mod config;

use carmen_db::collections::COLLECTION_EXTRACTION_CHAN;
use log::{error, info};
use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tokio::signal::unix::{SignalKind, signal};

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    let config = Config::load_env()?;
    let pool = PgPool::connect(&config.postgres_url).await?;
    info!("Database connection established");

    let mut queue_listener = PgListener::connect_with(&pool).await?;
    queue_listener.listen(COLLECTION_EXTRACTION_CHAN).await?;
    info!("listening to PG channel '{COLLECTION_EXTRACTION_CHAN}'");

    loop {
        tokio::select! {
            _ = signal_terminate.recv() => {
                info!("received SIGTERM, shutting down");
                break;
            },

            _ = signal_interrupt.recv() => {
                info!("received SIGINT, shutting down");
                break;
            },

            notification = queue_listener.recv() => match notification {
                Ok(notification) => {
                    let collection_id = notification.payload();
                    info!("Received extraction task {collection_id}");
                }
                Err(err) => error!("{err}"),
            }
        }
    }

    pool.close().await;

    Ok(())
}
