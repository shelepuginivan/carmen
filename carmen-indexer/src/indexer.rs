use std::sync::Mutex;

use fastembed::{InitOptions, TextEmbedding};
use text_splitter::{Characters, MarkdownSplitter};

use crate::config::Config;

pub struct Indexer {
    embedder: Mutex<TextEmbedding>,
    splitter: MarkdownSplitter<Characters>,
}

pub struct Chunk<'text> {
    pub text: &'text str,
    pub embedding: Vec<f32>,
}

impl Indexer {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let mut options = InitOptions::new(config.embedding_model.clone());

        if let Some(intra_threads) = config.embedding_threads {
            options = options.with_intra_threads(intra_threads);
        }

        let embedder = Mutex::new(TextEmbedding::try_new(options)?);
        let splitter = MarkdownSplitter::new(config.max_chunk_size);

        Ok(Self { embedder, splitter })
    }

    pub fn embed_document<'text>(&self, text: &'text str) -> anyhow::Result<Vec<Chunk<'text>>> {
        let fragments: Vec<&str> = self.splitter.chunks(text).collect();
        let embeddings = self.embedder.lock().unwrap().embed(&fragments, None)?;

        Ok(embeddings
            .into_iter()
            .zip(fragments)
            .map(|(embedding, text)| Chunk { embedding, text })
            .collect())
    }
}
