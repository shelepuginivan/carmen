# extractor

Service `extractor` extracts text from documents, splits text into chunks and
enqueues these chunks for embedding generation.

Documents are enqueued for extraction by the search service when a new document
is uploaded. Contents of the documents are stored in an S3-compatible object
storage.

## Environment variables

| Variable                                       | Description                   | Required                | Default     | Example              |
| ---------------------------------------------- | ----------------------------- | ----------------------- | ----------- | -------------------- |
| `CARMEN_EXTRACTOR_S3_ENDPOINT`                 | Object storage endpoint       | Yes                     | -           | `http://rustfs:9000` |
| `CARMEN_EXTRACTOR_S3_REGION`                   | Object storage region         | Yes                     | -           | `eu-central-1`       |
| `CARMEN_EXTRACTOR_S3_BUCKET`                   | Object storage bucket         | Yes                     | -           | `carmen-documents`   |
| `CARMEN_EXTRACTOR_S3_ACCESS_KEY`               | Object storage access key     | Yes                     | -           | `838296...4e56b0`    |
| `CARMEN_EXTRACTOR_S3_SECRET_KEY`               | Object storage secret key     | Yes                     | -           | `4756d4...b42529`    |
| `CARMEN_EXTRACTOR_KAFKA_URI`                   | URI of the Kafka broker       | Yes                     | -           | `kafka:9092`         |
| `CARMEN_EXTRACTOR_KAFKA_CONSUMER_GROUP`        | Kafka consumer group          | For service replication | `extractor` | `my-consumer-group`  |
| `CARMEN_EXTRACTOR_KAFKA_TOPIC_DOCUMENTS_QUEUE` | Documents queue topic (input) | Yes                     | -           | `documents.queue`    |
| `CARMEN_EXTRACTOR_KAFKA_TOPIC_CHUNKS_QUEUE`    | Chunks queue topic (output)   | Yes                     | -           | `chunks.queue`       |


## Example Docker Compose setup

```yaml
services:
  extractor:
    container_name: carmen-extractor
    build:
      context: ./extractor
    environment:
      CARMEN_EXTRACTOR_S3_ENDPOINT: http://rustfs:9000
      CARMEN_EXTRACTOR_S3_REGION: eu-central-1
      CARMEN_EXTRACTOR_S3_BUCKET: carmen-documents
      CARMEN_EXTRACTOR_S3_ACCESS_KEY: changme
      CARMEN_EXTRACTOR_S3_SECRET_KEY: changme
      CARMEN_EXTRACTOR_KAFKA_URI: kafka-broker:9092
      CARMEN_EXTRACTOR_KAFKA_CONSUMER_GROUP: extractor
      CARMEN_EXTRACTOR_KAFKA_TOPIC_DOCUMENTS_QUEUE: documents.queue
      CARMEN_EXTRACTOR_KAFKA_TOPIC_CHUNKS_QUEUE: chunks.queue
    restart: unless-stopped
    depends_on:
      kafka-init:
      kafka-broker:
      rustfs:
```
