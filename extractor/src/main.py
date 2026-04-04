import logging

from models import Config
from processor import DocumentProcessor
from s3 import DocumentsBucket


def main() -> None:
    logging.basicConfig(level=logging.WARNING)
    config = Config()  # type: ignore
    bucket = DocumentsBucket(config)
    processor = DocumentProcessor(bucket, config)
    processor.handle()


if __name__ == "__main__":
    main()
