use std::sync::Mutex;

use fastembed::{InitOptions, TextEmbedding};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use text_splitter::{Characters, MarkdownSplitter};

use crate::config::Config;

pub struct Indexer {
    embedder: Mutex<TextEmbedding>,
    splitter: MarkdownSplitter<Characters>,
    detector: LanguageDetector,
}

pub struct Chunk<'text> {
    pub text: &'text str,
    pub embedding: Vec<f32>,
    pub language: String,
}

impl Indexer {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let mut options = InitOptions::new(config.embedding_model.clone());

        if let Some(intra_threads) = config.embedding_threads {
            options = options.with_intra_threads(intra_threads);
        }

        let embedder = Mutex::new(TextEmbedding::try_new(options)?);
        let splitter = MarkdownSplitter::new(config.max_chunk_size);
        let detector = LanguageDetectorBuilder::from_languages(&config.languages).build();

        Ok(Self {
            embedder,
            splitter,
            detector,
        })
    }

    pub fn embed_document<'text>(&self, text: &'text str) -> anyhow::Result<Vec<Chunk<'text>>> {
        let fragments: Vec<&str> = self.splitter.chunks(text).collect();
        let embeddings = self.embedder.lock().unwrap().embed(&fragments, None)?;

        Ok(embeddings
            .into_iter()
            .zip(fragments)
            .map(|(embedding, text)| self.new_chunk(embedding, text))
            .collect())
    }

    fn new_chunk<'text>(&self, embedding: Vec<f32>, text: &'text str) -> Chunk<'text> {
        let language = self
            .detector
            .detect_language_of(text)
            .map(|lang| lang.to_string())
            .unwrap_or_else(|| "simple".to_string());

        Chunk {
            text,
            embedding,
            language,
        }
    }
}
