from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    sentence_transformers_home: str | None = None
    sentence_transformers_model: str
    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_chunks_queue: str = "chunks.queue"
    kafka_topic_chunks_ready: str = "chunks.ready"
    kafka_topic_search_requests: str = "search.requests"
    kafka_topic_search_responses: str = "search.responses"

    model_config = SettingsConfigDict(env_prefix="CARMEN_EMBEDDING_")
