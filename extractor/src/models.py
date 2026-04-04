from pydantic import BaseModel
from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    s3_endpoint: str
    s3_region: str
    s3_bucket: str
    s3_access_key: str
    s3_secret_key: str
    kafka_uri: str
    kafka_consumer_group: str
    kafka_topic_documents_queue: str
    kafka_topic_chunks_queue: str

    model_config = SettingsConfigDict(env_prefix="CARMEN_EXTRACTOR_")


class Document(BaseModel):
    document_id: str
    object_key: str
    mimetype: str


class Chunk(BaseModel):
    document_id: str
    text: str
