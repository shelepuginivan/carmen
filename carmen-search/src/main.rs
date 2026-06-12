use axum::Router;
use log::info;
use sqlx::PgPool;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod routers;
mod service;

use crate::config::Config;
use crate::routers::apidoc;
use crate::routers::collections;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let config = Config::load_env()?;
    let pool = PgPool::connect(&config.postgres_url).await?;
    info!("Database connection established");

    let mut app = Router::new()
        .nest("/api/v1/collections", collections::router())
        .with_state(pool); // TODO: wrap in AppState struct

    if let Some(docs_path) = config.docs_path {
        info!("API docs is available at {docs_path}");
        app = app.merge(SwaggerUi::new(docs_path).url("/openapi.json", apidoc::ApiDoc::openapi()));
    }

    let listener = TcpListener::bind(&config.http_addr).await?;
    info!("Listening {}...", config.http_addr);

    Ok(axum::serve(listener, app).await?)
}
