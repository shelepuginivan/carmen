import logging
import signal

from models import Config
from processor import DocumentProcessor
from s3 import DocumentsBucket


def main() -> None:
    logging.basicConfig(level=logging.INFO)

    config = Config()  # type: ignore
    bucket = DocumentsBucket(config)
    processor = DocumentProcessor(bucket, config)

    signal.signal(signal.SIGINT, processor.close)
    signal.signal(signal.SIGTERM, processor.close)

    processor.listen()


if __name__ == "__main__":
    main()
