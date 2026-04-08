import logging
import signal

from common.config import Config
from common.embedding import EmbeddingService

from .models import ProcessorConfig
from .processor import ChunkProcessor


def main() -> None:
    logging.basicConfig(level=logging.INFO)
    common_config = Config()  # type: ignore
    processor_config = ProcessorConfig()  # type: ignore
    service = EmbeddingService(common_config)
    processor = ChunkProcessor(processor_config, service)

    signal.signal(signal.SIGINT, processor.close)
    signal.signal(signal.SIGTERM, processor.close)

    processor.listen()
