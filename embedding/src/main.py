import logging
from concurrent.futures import ProcessPoolExecutor

from sentence_transformers import SentenceTransformer

from adapters.chunks import ChunkAdapter
from adapters.search import SearchAdapter
from models.config import Config


def main():
    logging.basicConfig(level=logging.INFO)
    config = Config()  # type: ignore
    transformer = SentenceTransformer(config.sentence_transformer)

    chunk_adapter = ChunkAdapter(config, transformer)
    search_adapter = SearchAdapter(config, transformer)

    with ProcessPoolExecutor(max_workers=2) as executor:
        executor.submit(chunk_adapter.handle)
        executor.submit(search_adapter.handle)


if __name__ == "__main__":
    main()
