from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    sentence_transformer: str
    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_chunks_queue: str = "chunks.queue"
    kafka_topic_chunks_ready: str = "chunks.ready"

    model_config = SettingsConfigDict(env_prefix="CARMEN_EMBEDDING_")
