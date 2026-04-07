import logging
from typing import Any

from kafka import KafkaConsumer, KafkaProducer
from langchain_text_splitters import MarkdownTextSplitter

from models import Chunk, Config, Document
from s3 import DocumentsBucket


class DocumentProcessor:
    def __init__(self, bucket: DocumentsBucket, config: Config) -> None:
        self.__bucket = bucket
        self.__topic_chunks_queue = config.kafka_topic_chunks_queue
        self.__consumer = KafkaConsumer(
            config.kafka_topic_documents_queue,
            bootstrap_servers=config.kafka_uri,
            group_id=config.kafka_consumer_group,
            auto_offset_reset="earliest",
            enable_auto_commit=True,
        )
        self.__producer = KafkaProducer(
            bootstrap_servers=config.kafka_uri,
            value_serializer=self.__encode_result,
        )
        self.__splitter = MarkdownTextSplitter(
            chunk_size=500,
            chunk_overlap=100,
        )

    def __del__(self) -> None:
        self.__consumer.close()
        self.__producer.close()

    def handle(self) -> None:
        for message in map(self.__decode_message, self.__consumer):
            print(message)
            try:
                self.__handle_msg(message)
            except Exception as err:
                logging.error(err)

    def __handle_msg(self, doc: Document) -> None:
        content = self.__bucket.get_object(doc.document_id).decode("utf-8")

        for chunk_text in self.__splitter.split_text(content):
            chunk = Chunk(document_id=doc.document_id, text=chunk_text)
            self.__producer.send(self.__topic_chunks_queue, chunk)

    def __decode_message(self, message: Any) -> Document:
        return Document.model_validate_json(message.value)

    def __encode_result(self, chunk: Chunk) -> bytes:
        return chunk.model_dump_json().encode("utf-8")
