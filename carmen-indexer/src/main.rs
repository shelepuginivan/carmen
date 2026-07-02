use std::time::Duration;

use carmen_db::documents::DocumentIndexing;
use log::{error, info};
use tokio::signal::unix::{SignalKind, signal};
use tokio::time::{MissedTickBehavior, interval};

mod config;
mod worker;

use crate::config::Config;
use crate::worker::WorkerHandle;

const INDEXING_POLL_INTERVAL: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    let config = Config::load_env()?;
    let pool = carmen_db::connect_from_env().await?;

    let mut interval = interval(INDEXING_POLL_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let worker = WorkerHandle::new(&config, pool.clone())?;

    loop {
        tokio::select! {
            _ = signal_terminate.recv() => {
                info!("Received SIGTERM, shutting down...");
                break;
            },

            _ = signal_interrupt.recv() => {
                info!("Received SIGINT, shutting down...");
                break;
            },

            _ = interval.tick() => match DocumentIndexing::claim(&pool).await {
                Ok(Some(indexing)) => {
                    worker.push_indexing(indexing).await;
                    interval.reset_immediately();
                },
                Ok(None) => {}
                Err(err) => error!("Failed to claim indexing: {err}"),
            },
        }
    }

    worker.stop().await?;
    pool.clone().close().await;

    Ok(())
}
