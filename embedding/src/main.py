import logging
import signal
from multiprocessing import Process

from adapters.chunks import ChunkAdapter
from adapters.search import SearchAdapter
from models.config import Config
from service.embedding import EmbeddingService


def run_chunk_adapter(config: Config) -> None:
    service = EmbeddingService(config.sentence_transformers)
    adapter = ChunkAdapter(config, service)
    logging.info("Starting chunk adapter...")
    adapter.handle()


def run_search_adapter(config: Config) -> None:
    service = EmbeddingService(config.sentence_transformers)
    adapter = SearchAdapter(config, service)
    logging.info("Starting search adapter...")
    adapter.handle()


def main():
    logging.basicConfig(level=logging.INFO)
    config = Config()  # type: ignore

    chunks = Process(target=run_chunk_adapter, args=[config])
    search = Process(target=run_search_adapter, args=[config])

    def signal_handler(signum: int, _) -> None:
        logging.info(f"Received signal {signum}, shutting down...")
        chunks.terminate()
        search.terminate()

    chunks.start()
    search.start()

    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    signal.pause()

    chunks.join()
    search.join()


if __name__ == "__main__":
    main()
