use carmen_db::extractions::Extraction;
use sqlx::PgPool;
use uuid::Uuid;

use super::error::Result;

pub mod dto;

#[derive(Clone)]
pub struct ExtractionService {
    pool: PgPool,
}

impl ExtractionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_for_collection(&self, id: Uuid) -> Result<Vec<dto::Extraction>> {
        Ok(Extraction::get_for_collection(&self.pool, id)
            .await?
            .into_iter()
            .map(dto::Extraction::from)
            .collect())
    }

    pub async fn schedule(
        &self,
        dto::ScheduleExtraction {
            collection_id,
            source,
            source_type,
            parameters,
            extraction_type,
        }: dto::ScheduleExtraction,
    ) -> Result<dto::Extraction> {
        Ok(Extraction::schedule(
            &self.pool,
            collection_id,
            &source,
            &source_type,
            extraction_type.into(),
            &parameters,
        )
        .await?
        .into())
    }

    pub async fn cancel(&self, id: Uuid) -> Result<dto::CancellationResult> {
        let cancelled = Extraction::cancel(&self.pool, id).await?;
        Ok(dto::CancellationResult { cancelled })
    }

    pub async fn replay(&self, id: Uuid) -> Result<dto::Extraction> {
        let extraction = Extraction::get_by_id(&self.pool, id).await?;
        let replay = Extraction::schedule(
            &self.pool,
            extraction.collection_id,
            &extraction.source,
            &extraction.source_type,
            extraction.extraction_type,
            &extraction.parameters,
        )
        .await?;

        Ok(replay.into())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<dto::Extraction> {
        let deleted = Extraction::get_by_id(&self.pool, id).await?;
        Ok(deleted.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<dto::Extraction> {
        let deleted = Extraction::delete(&self.pool, id).await?;
        Ok(deleted.into())
    }
}
