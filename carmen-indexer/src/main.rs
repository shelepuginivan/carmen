use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tokio::signal::unix::{SignalKind, signal};

mod config;

use crate::config::Config;

const CHANNEL: &str = "indexing";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load_env()?;
    let pool = PgPool::connect(&config.postgres_url).await?;
    let mut listener = PgListener::connect_with(&pool).await?;

    let mut signal_terminate = signal(SignalKind::terminate())?;
    let mut signal_interrupt = signal(SignalKind::interrupt())?;

    listener.listen(CHANNEL).await?;

    loop {
        tokio::select! {
            _ = signal_terminate.recv() => break,
            _ = signal_interrupt.recv() => break,

            notification = listener.recv() => match notification {
                Ok(notification) => {
                    println!("{}", notification.payload());
                }
                Err(err) => {
                    break;
                }
            }
        }
    }

    listener.unlisten(CHANNEL).await?;

    Ok(())
}
