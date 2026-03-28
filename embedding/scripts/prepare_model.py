from pydantic_settings import BaseSettings
from sentence_transformers import SentenceTransformer


class Config(BaseSettings):
    sentence_transformers_home: str
    sentence_transformers_models: list[str]


config = Config()  # type: ignore


for model in config.sentence_transformers_models:
    # NOTE: sentence_transformers uses SENTENCE_TRANSFORMERS_HOME environment
    #       variable for cache, which is defined in Containerfile.
    SentenceTransformer(model)
