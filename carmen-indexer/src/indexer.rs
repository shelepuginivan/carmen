use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use text_splitter::{Characters, MarkdownSplitter};

pub struct Indexer {
    embedder: TextEmbedding,
    splitter: MarkdownSplitter<Characters>,
}

pub struct Chunk<'a> {
    pub text: &'a str,
    pub embedding: Vec<f32>,
}

impl Indexer {
    pub fn new() -> anyhow::Result<Self> {
        // TODO: configure from env vars, maybe move to common crate?
        //       also download models at build time?
        let embedder = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_intra_threads(4),
        )?;

        let splitter = MarkdownSplitter::new(512);

        Ok(Self { embedder, splitter })
    }

    pub fn embed_document<'text>(&mut self, text: &'text str) -> anyhow::Result<Vec<Chunk<'text>>> {
        let fragments: Vec<&str> = self.splitter.chunks(text).collect();
        let embeddings = self.embedder.embed(&fragments, None)?;

        Ok(embeddings
            .into_iter()
            .zip(fragments)
            .map(|(embedding, text)| Chunk { embedding, text })
            .collect())
    }
}
