use s3::Bucket;
use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub bucket: Box<Bucket>,
}

impl AppState {
    pub fn new(db: PgPool, bucket: Box<Bucket>) -> Self {
        Self { db, bucket }
    }
}
