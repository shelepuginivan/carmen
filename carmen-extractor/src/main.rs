mod config;

use carmen_db::collections::{COLLECTION_EXTRACTION_CHAN, Collection, CollectionExtraction};
use log::{error, info};
use sqlx::postgres::PgListener;
use sqlx::{PgPool, types::Uuid};
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
                    let payload = notification.payload().to_owned();
                    let pool = pool.clone();

                    tokio::spawn(async move {
                        let _ = exec_task(&pool, &payload).await;
                    });
                }
                Err(err) => error!("{err}"),
            }
        }
    }

    pool.clone().close().await;

    Ok(())
}

async fn exec_task(pool: &PgPool, payload: &str) -> anyhow::Result<()> {
    let task_id: Uuid = payload.parse()?;
    info!("Received extraction task {task_id}");

    let task = match CollectionExtraction::claim(pool, task_id).await? {
        Some(claimed) => claimed,
        None => {
            info!("Task {task_id} is claimed by another extractor instance");
            return Ok(());
        }
    };

    let collection = Collection::get(pool, task.collection_id).await?;
    info!("Starting extraction of collection {}", collection.id);

    Ok(())
}
