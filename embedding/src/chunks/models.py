from pydantic import BaseModel
from pydantic_settings import BaseSettings, SettingsConfigDict


class ChunkEnqueued(BaseModel):
    document_id: str
    text: str


class ChunkReady(BaseModel):
    document_id: str
    text: str
    embedding: list[float]
    language: str


class ProcessorConfig(BaseSettings):
    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_chunks_queue: str = "chunks.queue"
    kafka_topic_chunks_ready: str = "chunks.ready"

    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        env_prefix="CARMEN_EMBEDDING_",
    )
