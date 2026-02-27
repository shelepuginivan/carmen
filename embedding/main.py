import json

from kafka import KafkaConsumer, KafkaProducer


producer = KafkaProducer(value_serializer=lambda v: json.dumps(v).encode("utf-8"))


def consume_embedding():
    consumer = KafkaConsumer("embedding.queue")

    for msg in consumer:
        print(msg.value)


def main():
    consume_embedding()


if __name__ == "__main__":
    main()
