from typing import Any

from kafka import KafkaConsumer, KafkaProducer
from sentence_transformers import SentenceTransformer

from models.config import Config
from models.search import SearchRequest, SearchResponse


class SearchAdapter:
    def __init__(self, config: Config) -> None:
        self.__config = config
        self.__transformer = SentenceTransformer(
            config.sentence_transformers_model,
            cache_folder=config.sentence_transformers_home,
            local_files_only=config.sentence_transformers_home is not None,
        )
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
        for message in map(self.__decode_message, self.__consumer):
            embedding = self.__transformer.encode(message.query).tolist()
            result = SearchResponse(embedding=embedding)
            self.__producer.send(self.__config.kafka_topic_search_responses, result)

    def __decode_message(self, message: Any) -> SearchRequest:
        return SearchRequest.model_validate_json(message.value)

    def __encode_result(self, result: SearchResponse) -> bytes:
        return result.model_dump_json().encode("utf-8")
