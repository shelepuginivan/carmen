use axum::Router;
use carmen_s3::Storage;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use log::info;
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app;
mod config;
mod routers;
mod service;

use crate::app::AppState;
use crate::config::Config;
use crate::routers::{apidoc, collections, search};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_LOG");

    let config = Config::load_env()?;
    let pool = carmen_db::connect_from_env().await?;
    let storage = Storage::new_from_env()?;

    // TODO: from env
    let embedder = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;

    let state = AppState::new(pool, storage, embedder);

    let mut app = Router::new()
        .nest("/api/v1/collections", collections::router())
        .nest("/api/v1/search", search::router())
        .with_state(state);

    if let Some(docs_path) = config.docs_path {
        info!("API docs is available at {docs_path}");
        app = app.merge(SwaggerUi::new(docs_path).url("/openapi.json", apidoc::ApiDoc::openapi()));
    }

    let listener = TcpListener::bind(&config.http_addr).await?;
    info!("Listening {}...", config.http_addr);

    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal(SignalKind::interrupt())
            .expect("failed to install SIGINT handler")
            .recv()
            .await
    };

    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => info!("Received SIGINT, shutting down..."),
        _ = terminate => info!("Received SIGTERM, shutting down..."),
    }
}
