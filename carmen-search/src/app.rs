use std::sync::{Arc, Mutex};

use carmen_s3::Storage;
use fastembed::TextEmbedding;
use sqlx::PgPool;

use crate::service::collections::CollectionService;
use crate::service::documents::DocumentsService;
use crate::service::search::SearchService;

#[derive(Clone)]
pub struct AppState {
    pub collections: CollectionService,
    pub documents: DocumentsService,
    pub search: SearchService,
}

impl AppState {
    pub fn new(pool: PgPool, storage: Storage, embedder: TextEmbedding) -> Self {
        let pool = Arc::new(pool);
        let storage = Arc::new(storage);
        let embedder = Arc::new(Mutex::new(embedder));

        let collections = CollectionService::new(pool.clone(), storage.clone());
        let documents = DocumentsService::new(pool.clone(), storage);
        let search = SearchService::new(pool, embedder);

        Self {
            collections,
            documents,
            search,
        }
    }
}
