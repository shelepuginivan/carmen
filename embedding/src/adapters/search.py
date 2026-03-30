from typing import Any

from kafka import KafkaConsumer, KafkaProducer

from models.config import Config
from models.search import SearchRequest, SearchResponse
from service.embedding import EmbeddingService


class SearchAdapter:
    def __init__(self, config: Config, service: EmbeddingService) -> None:
        self.__service = service
        self.__consumer = KafkaConsumer(
            config.kafka_topic_search_requests,
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
        for raw_msg in self.__consumer:
            message = self.__decode_message(raw_msg)
            r = self.__service.generate_embedding(message.query)

            result = SearchResponse(
                embedding=r.embedding.tolist(),
                language=r.language,
            )

            self.__producer.send(message.response_topic, result, key=raw_msg.key)

    def __decode_message(self, message: Any) -> SearchRequest:
        return SearchRequest.model_validate_json(message.value)

    def __encode_result(self, result: SearchResponse) -> bytes:
        return result.model_dump_json().encode("utf-8")
