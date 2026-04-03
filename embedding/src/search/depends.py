from fastapi import Depends

from models.config import Config
from service.embedding import EmbeddingService


def get_config() -> Config:
    return Config()  # type: ignore


def get_service(config=Depends(get_config)) -> EmbeddingService:
    return EmbeddingService(config)
