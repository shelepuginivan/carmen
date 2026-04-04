from pydantic import HttpUrl
from pydantic_settings import BaseSettings, SettingsConfigDict


class Config(BaseSettings):
    sentence_transformers_home: str | None = None
    model: dict[str, str]
    langdetector_url: HttpUrl

    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        env_prefix="CARMEN_EMBEDDING_",
    )
