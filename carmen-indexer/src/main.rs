use std::sync::Arc;

use carmen_db::documents::DOCUMENT_INDEXING_CHAN;
use log::{error, info};
use s3::creds::Credentials;
use s3::{Bucket, Region};
use sqlx::postgres::PgListener;
use sqlx::{PgPool, types::Uuid};
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::Semaphore;

mod config;
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
    queue_listener.listen(DOCUMENT_INDEXING_CHAN).await?;
    info!("listening to PG channel '{DOCUMENT_INDEXING_CHAN}'");

    let region = Region::Custom {
        region: config.s3_region,
        endpoint: config.s3_endpoint,
    };

    let credentials = Credentials::new(
        Some(&config.s3_access_key),
        Some(&config.s3_secret_key),
        None,
        None,
        None,
    )?;

    let bucket = Bucket::new(&config.s3_bucket, region, credentials)?.with_path_style();

    // Indexing is computationally heavy, hence limit the number of concurrent tasks.
    let tasks = Arc::new(Semaphore::new(config.task_limit));

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
                    let token = tasks.clone().acquire_owned().await.unwrap();

                    let task_id: Uuid = notification.payload().parse()?;
                    let pool = pool.clone();
                    let bucket = bucket.clone();
                    let (task, _cancel_tx) = Task::new(task_id, pool, bucket);

                    tokio::spawn(async move {
                        let _ = task.start().await;
                        drop(token)
                    });
                }
                Err(err) => error!("{err}"),
            }
        }
    }

    pool.clone().close().await;

    Ok(())
}
