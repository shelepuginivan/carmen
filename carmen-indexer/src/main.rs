use carmen_db::documents::DOCUMENT_INDEXING_CHAN;
use log::{error, info};
use sqlx::postgres::PgListener;
use sqlx::types::Uuid;
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;

mod config;
mod worker;

use crate::config::Config;
use crate::worker::{Task, Worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    let config = Config::load_env()?;
    let pool = carmen_db::connect_from_env().await?;

    let mut queue_listener = PgListener::connect_with(&pool).await?;
    queue_listener.listen(DOCUMENT_INDEXING_CHAN).await?;
    info!("listening to PG channel '{DOCUMENT_INDEXING_CHAN}'");

    let (tx, rx) = mpsc::channel(16);
    let worker = Worker::new(&config, pool.clone(), rx)?;
    let handle = tokio::spawn(worker.start());

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
                    let task = Task::new(task_id);
                    tx.send(task).await?;
                }
                Err(err) => error!("{err}"),
            }
        }
    }

    drop(tx);

    handle.await?;

    queue_listener.unlisten_all().await?;
    pool.clone().close().await;

    Ok(())
}
