from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    kafka_uri: str
    kafka_consumer_group: str = "embedding-consumer-group"
    kafka_topic_embedding_queue: str = "embedding.queue"

    model_config = SettingsConfigDict(env_prefix="CARMEN_EMBEDDING_")
