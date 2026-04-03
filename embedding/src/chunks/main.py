import logging

from .processor import ChunkProcessor
from models.config import Config
from service.embedding import EmbeddingService


def main() -> None:
    config = Config()  # type: ignore
    service = EmbeddingService(config)
    processor = ChunkProcessor(config, service)
    logging.info("Starting chunk processor...")
    processor.handle()
