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

    pub async fn get(&self, id: Uuid) -> Result<dto::Extraction> {
        let deleted = Extraction::get(&self.pool, id).await?;
        Ok(deleted.into())
    }

    pub async fn get_by_collection_id(&self, id: Uuid) -> Result<Vec<dto::Extraction>> {
        Ok(Extraction::get_by_collection_id(&self.pool, id)
            .await?
            .into_iter()
            .map(dto::Extraction::from)
            .collect())
    }

    pub async fn schedule(
        &self,
        collection_id: Uuid,
        v: dto::ScheduleExtraction,
    ) -> Result<dto::Extraction> {
        Ok(Extraction::schedule(
            &self.pool,
            collection_id,
            &v.source,
            &v.source_type,
            v.extraction_type.into(),
            &v.parameters,
        )
        .await?
        .into())
    }

    pub async fn bulk_schedule(
        &self,
        collection_id: Uuid,
        v: dto::BulkScheduleExtraction,
    ) -> Result<()> {
        Extraction::bulk_schedule(
            &self.pool,
            collection_id,
            &v.source,
            &v.source_type,
            v.extraction_type.into(),
            &v.parameters,
        )
        .await?;
        Ok(())
    }

    pub async fn cancel(&self, id: Uuid) -> Result<dto::CancellationResult> {
        let cancelled = Extraction::cancel(&self.pool, id).await?;
        Ok(dto::CancellationResult { cancelled })
    }

    pub async fn replay(&self, id: Uuid) -> Result<dto::Extraction> {
        let extraction = Extraction::get(&self.pool, id).await?;
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

    pub async fn delete(&self, id: Uuid) -> Result<dto::Extraction> {
        let deleted = Extraction::delete(&self.pool, id).await?;
        Ok(deleted.into())
    }
}
