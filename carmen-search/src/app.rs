use std::sync::Arc;

use carmen_s3::Storage;
use sqlx::PgPool;

use crate::service::collections::CollectionService;
use crate::service::documents::DocumentsService;

#[derive(Clone)]
pub struct AppState {
    pub collections: CollectionService,
    pub documents: DocumentsService,
}

impl AppState {
    pub fn new(pool: PgPool, storage: Storage) -> Self {
        let pool = Arc::new(pool);
        let storage = Arc::new(storage);

        let collections = CollectionService::new(pool.clone(), storage.clone());
        let documents = DocumentsService::new(pool, storage);

        Self {
            collections,
            documents,
        }
    }
}
