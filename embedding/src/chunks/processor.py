from typing import Any

from kafka import KafkaConsumer, KafkaProducer

from models.config import Config
from models.chunks import ChunkEnqueued, ChunkReady
from service.embedding import EmbeddingService


class ChunkProcessor:
    def __init__(self, config: Config, service: EmbeddingService) -> None:
        self.__config = config
        self.__service = service
        self.__consumer = KafkaConsumer(
            config.kafka_topic_chunks_queue,
            bootstrap_servers=config.kafka_uri,
            group_id=config.kafka_consumer_group,
            auto_offset_reset="earliest",
            enable_auto_commit=True,
        )
        self.__producer = KafkaProducer(
            bootstrap_servers=config.kafka_uri,
            value_serializer=self.__encode_result,
        )

    def __del__(self) -> None:
        self.__consumer.close()
        self.__producer.close()

    def handle(self) -> None:
        for message in map(self.__decode_message, self.__consumer):
            r = self.__service.generate_embedding(message.text)

            result = ChunkReady(
                document_id=message.document_id,
                text=message.text,
                embedding=r.embedding,
                language=r.language,
            )

            self.__producer.send(self.__config.kafka_topic_chunks_ready, result)

    def __decode_message(self, message: Any) -> ChunkEnqueued:
        return ChunkEnqueued.model_validate_json(message.value)

    def __encode_result(self, result: ChunkReady) -> bytes:
        return result.model_dump_json().encode("utf-8")
