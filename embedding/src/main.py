import json
import logging

from kafka import KafkaConsumer, KafkaProducer

from config import Config


def consume_embedding(config: Config):
    consumer = KafkaConsumer(
        config.kafka_topic_chunks_queue,
        bootstrap_servers=config.kafka_uri,
        group_id=config.kafka_consumer_group,
        auto_offset_reset="earliest",
        value_deserializer=lambda v: v.decode("utf-8"),
        enable_auto_commit=True,
    )

    logging.info(f"Subscribed to {config.kafka_topic_chunks_queue}")

    for message in consumer:
        logging.info(message.value)

    consumer.close()


def main():
    logging.basicConfig(level=logging.INFO)
    config = Config()  # type: ignore

    producer = KafkaProducer(
        bootstrap_servers=config.kafka_uri,
        value_serializer=lambda v: json.dumps(v).encode("utf-8"),
    )

    consume_embedding(config)


if __name__ == "__main__":
    main()
