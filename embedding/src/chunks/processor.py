import logging

from common.embedding import EmbeddingService
from confluent_kafka import Consumer, Producer

from .models import ChunkEnqueued, ChunkReady, ProcessorConfig


class ChunkProcessor:
    def __init__(self, config: ProcessorConfig, service: EmbeddingService) -> None:
        self._running = True
        self._topic_chunks_ready = config.kafka_topic_chunks_ready
        self._service = service
        self._producer = Producer({"bootstrap.servers": config.kafka_uri})
        self._consumer = Consumer(
            {
                "bootstrap.servers": config.kafka_uri,
                "group.id": config.kafka_consumer_group,
                "enable.auto.commit": True,
                "auto.offset.reset": "earliest",
            }
        )
        self._consumer.subscribe([config.kafka_topic_chunks_queue])

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

        chunk = ChunkEnqueued.model_validate_json(msg)

        logging.info(
            f"processing chunk #{chunk.index} of document {chunk.document_id}..."
        )

        r = self._service.generate_embedding(chunk.text)

        result = ChunkReady(
            document_id=chunk.document_id,
            text=chunk.text,
            embedding=r.embedding,
            language=r.language,
        )

        self._producer.produce(
            topic=self._topic_chunks_ready,
            value=result.model_dump_json().encode("utf-8"),
        )

        logging.info(f"finished processing chunk #{chunk.index}")
