from pydantic_settings import BaseSettings, SettingsConfigDict


class SentenceTransformersConfig:
    sentence_transformers_home: str | None = None
    sentence_transformers_model: dict[str, str]


class Config(BaseSettings):
    sentence_transformers_home: str | None = None
    sentence_transformers_model: str
    sentence_transformers: SentenceTransformersConfig
    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_chunks_queue: str = "chunks.queue"
    kafka_topic_chunks_ready: str = "chunks.ready"
    kafka_topic_search_requests: str = "search.requests"

    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        env_prefix="CARMEN_EMBEDDING_",
    )
