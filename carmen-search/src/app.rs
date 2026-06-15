use carmen_s3::Storage;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub storage: Storage,
}

impl AppState {
    pub fn new(db: PgPool, storage: Storage) -> Self {
        Self { db, storage }
    }
}
