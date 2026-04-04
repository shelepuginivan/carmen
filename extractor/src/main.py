from .models import Config
from .processor import DocumentProcessor
from .s3 import DocumentsBucket


def main() -> None:
    config = Config()  # type: ignore
    bucket = DocumentsBucket(config)
    processor = DocumentProcessor(bucket, config)
    processor.handle()
