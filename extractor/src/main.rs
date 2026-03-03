use extractor::adapter::DocumentAdapter;
use extractor::config::Config;
use extractor::storage::DocumentStorage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_EXTRACTOR_LOG");

    let config = Config::read_from_env()?;
    let storage = DocumentStorage::new(&config)?;
    let document_adapter = DocumentAdapter::new(&config, storage)?;

    document_adapter.handle().await;

    Ok(())
}
