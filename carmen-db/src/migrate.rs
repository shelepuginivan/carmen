use std::env;

use anyhow::Context;
use log::info;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let postgres_url =
        env::var("CARMEN_POSTGRES_URL").context("env variable CARMEN_POSTGRES_URL is required")?;

    let pool = PgPool::connect(&postgres_url).await?;
    info!("Connected to database");

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Successfully applied migrations");

    Ok(())
}
