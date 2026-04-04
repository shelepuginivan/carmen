import logging

from common.config import Config
from common.embedding import EmbeddingService

from .models import ProcessorConfig
from .processor import ChunkProcessor


def main() -> None:
    common_config = Config()  # type: ignore
    processor_config = ProcessorConfig()  # type: ignore
    service = EmbeddingService(common_config)
    processor = ChunkProcessor(processor_config, service)
    logging.info("Starting chunk processor...")
    processor.handle()
