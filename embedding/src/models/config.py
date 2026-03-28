from lingua import Language
from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    sentence_transformers_home: str | None = None
    model: dict[str, str]
    fallback_language: str | None = None

    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_chunks_queue: str = "chunks.queue"
    kafka_topic_chunks_ready: str = "chunks.ready"
    kafka_topic_search_requests: str = "search.requests"

    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        env_prefix="CARMEN_EMBEDDING_",
    )

    def get_fallback_language(self) -> Language | None:
        if self.fallback_language is None:
            return None
        try:
            return Language.from_str(self.fallback_language)
        except ValueError:
            return None
