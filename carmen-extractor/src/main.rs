use std::time::Duration;

use carmen_db::extractions::Extraction;
use log::{error, info};
use tokio::signal::unix::{SignalKind, signal};
use tokio::time::{MissedTickBehavior, interval};

mod document;
mod documents;
mod extractors;
mod worker;

use crate::worker::WorkerHandle;

const EXTRACTION_POLL_INTERVAL: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    let pool = carmen_db::connect_from_env().await?;

    let mut interval = interval(EXTRACTION_POLL_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let worker = WorkerHandle::new(pool.clone())?;

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

            _ = interval.tick() => match Extraction::claim(&pool).await {
                Ok(Some(extraction)) => {
                    worker.push_extraction(extraction).await;
                    interval.reset_immediately();
                },
                Ok(None) => {}
                Err(err) => error!("Failed to claim extraction: {err}"),
            },
        }
    }

    worker.stop().await?;
    pool.clone().close().await;

    Ok(())
}
