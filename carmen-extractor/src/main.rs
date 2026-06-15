use carmen_db::collections::COLLECTION_EXTRACTION_CHAN;
use carmen_s3::Storage;
use log::{error, info};
use sqlx::postgres::PgListener;
use sqlx::{PgPool, types::Uuid};
use tokio::signal::unix::{SignalKind, signal};

mod config;
mod document;
mod documents;
mod extractors;
mod task;

use crate::config::Config;
use crate::task::Task;

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

    let storage = Storage::new_from_env()?;

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
                    let task_id: Uuid = notification.payload().parse()?;
                    let pool = pool.clone();
                    let storage = storage.clone();
                    let (task, _cancel_tx) = Task::new(task_id, pool, storage);

                    tokio::spawn(async move {
                        let _ = task.start().await;
                    });
                }
                Err(err) => error!("{err}"),
            }
        }
    }

    pool.clone().close().await;

    Ok(())
}
