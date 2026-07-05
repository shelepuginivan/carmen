use std::error::Error;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env("CARMEN_LOG");

    let pool = carmen_db::connect_from_env().await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Successfully applied migrations");

    pool.close().await;

    Ok(())
}
