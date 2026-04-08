import logging

from confluent_kafka import Consumer, Producer
from langchain_text_splitters import MarkdownTextSplitter

from models import Chunk, Config, Document
from s3 import DocumentsBucket


class DocumentProcessor:
    def __init__(self, bucket: DocumentsBucket, config: Config) -> None:
        self._running = True
        self._bucket = bucket
        self._topic_chunks_queue = config.kafka_topic_chunks_queue
        self._splitter = MarkdownTextSplitter(
            chunk_size=500,
            chunk_overlap=100,
        )
        self._producer = Producer({"bootstrap.servers": config.kafka_uri})
        self._consumer = Consumer(
            {
                "bootstrap.servers": config.kafka_uri,
                "group.id": config.kafka_consumer_group,
                "enable.auto.commit": True,
                "auto.offset.reset": "earliest",
            }
        )
        self._consumer.subscribe([config.kafka_topic_documents_queue])

    def close(self, *_) -> None:
        logging.info("shutting down...")
        self._running = False

    def listen(self) -> None:
        logging.info("listening for incoming messages...")

        while self._running:
            msg = self._consumer.poll(1.0)
            if msg is None:
                continue
            err = msg.error()
            if err is not None:
                logging.error(f"consumer error: {err}")

            self._handle_msg(msg.value())

        self._producer.flush()
        self._consumer.close()

    def _handle_msg(self, msg: bytes | None) -> None:
        if msg is None:
            return

        document = Document.model_validate_json(msg)
        logging.info(f"processing document {document.id}...")
        content = self._bucket.get_object(document.id).decode("utf-8")

        for i, chunk_text in enumerate(self._splitter.split_text(content)):
            chunk = Chunk(
                document_id=document.id,
                index=i + 1,
                text=chunk_text,
            )

            self._producer.produce(
                topic=self._topic_chunks_queue,
                value=chunk.model_dump_json().encode("utf-8"),
            )

        logging.info(f"finished processing document {document.id}")
