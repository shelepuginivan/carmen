from common.config import Config
from common.embedding import EmbeddingService
from fastapi import Depends


def get_config() -> Config:
    return Config()  # type: ignore


def get_service(config=Depends(get_config)) -> EmbeddingService:
    return EmbeddingService(config)
