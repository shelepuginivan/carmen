use env_logger;
use log::{error, info};
use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tokio::signal::unix::{SignalKind, signal};

mod config;

use crate::config::Config;

const CHANNEL: &str = "indexing";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let config = Config::load_env()?;
    let pool = PgPool::connect(&config.postgres_url).await?;
    let mut listener = PgListener::connect_with(&pool).await?;

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    listener.listen(CHANNEL).await?;
    info!("listening to PG channel '{CHANNEL}'");

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

            notification = listener.recv() => match notification {
                Ok(notification) => {
                    info!("{}", notification.payload());
                }
                Err(err) => {
                    error!("{err}");
                    break;
                }
            }
        }
    }

    listener.unlisten(CHANNEL).await?;
    info!("stopped listening to PG channel '{CHANNEL}'");

    Ok(())
}
