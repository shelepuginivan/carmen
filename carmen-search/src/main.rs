use axum::Router;
use axum::routing::get;
use log::info;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod config;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let config = Config::load_env()?;
    let pool = PgPool::connect(&config.postgres_url).await?;
    info!("Database connection established");

    let app = Router::new().route("/health", get(health)).with_state(pool);
    let listener = TcpListener::bind(&config.http_addr).await?;
    info!("Listening {}...", config.http_addr);

    Ok(axum::serve(listener, app).await?)
}

async fn health() -> String {
    "all systems operational".to_owned()
}
