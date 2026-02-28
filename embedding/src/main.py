import logging

from sentence_transformers import SentenceTransformer

from adapters.chunks import ChunkAdapter
from models.config import Config


def main():
    logging.basicConfig(level=logging.INFO)
    config = Config()  # type: ignore

    transformer = SentenceTransformer(config.sentence_transformer)

    adapter = ChunkAdapter(config, transformer)
    adapter.handle()


if __name__ == "__main__":
    main()
