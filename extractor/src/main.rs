use extractor::adapter::DocumentAdapter;
use extractor::config::Config;
use extractor::extractors::{MarkdownExtractor, PlaintextExtractor};
use extractor::processor::Processor;
use extractor::storage::DocumentStorage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_EXTRACTOR_LOG");

    let mut processor = Processor::default();
    processor.register_extractor(Box::new(MarkdownExtractor::default()));
    processor.register_extractor(Box::new(PlaintextExtractor::default()));

    let config = Config::read_from_env()?;
    let storage = DocumentStorage::new(&config)?;
    let document_adapter = DocumentAdapter::new(&config, storage, processor)?;

    document_adapter.handle().await;

    Ok(())
}
