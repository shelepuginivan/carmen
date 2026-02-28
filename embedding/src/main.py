import logging

from adapters.chunks import ChunkAdapter
from models.config import Config


def main():
    logging.basicConfig(level=logging.INFO)
    config = Config()  # type: ignore
    adapter = ChunkAdapter(config)
    adapter.handle()


if __name__ == "__main__":
    main()
