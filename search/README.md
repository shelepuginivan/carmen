# search

Service `search` provides searching capabilities via REST API.

## Environment variables

| Variable                                    | Description                          | Required                | Default           | Example                        |
| ------------------------------------------- | ------------------------------------ | ----------------------- | ----------------- | ------------------------------ |
| `CARMEN_SEARCH_SERVER_ADDR`                 | HTTP bind address                    | Yes                     | `:8000`           | `0.0.0.0:80`                   |
| `CARMEN_SEARCH_POSTGRES_DB`                 | PostgreSQL database                  | Yes                     | -                 | `carmen`                       |
| `CARMEN_SEARCH_POSTGRES_HOST`               | PostgreSQL host                      | Yes                     | -                 | `postgres`                     |
| `CARMEN_SEARCH_POSTGRES_PORT`               | PostgreSQL port                      | Yes                     | -                 | `5432`                         |
| `CARMEN_SEARCH_POSTGRES_USER`               | PostgreSQL user                      | Yes                     | -                 | `postgres`                     |
| `CARMEN_SEARCH_POSTGRES_PASSWORD`           | PostgreSQL password                  | Yes                     | -                 | `123456`                       |
| `CARMEN_SEARCH_S3_ENDPOINT`                 | Object storage endpoint              | Yes                     | -                 | `http://rustfs:9000`           |
| `CARMEN_SEARCH_S3_REGION`                   | Object storage region                | Yes                     | -                 | `eu-central-1`                 |
| `CARMEN_SEARCH_S3_BUCKET`                   | Object storage bucket                | Yes                     | -                 | `carmen-documents`             |
| `CARMEN_SEARCH_S3_ACCESS_KEY`               | Object storage access key            | Yes                     | -                 | `838296...4e56b0`              |
| `CARMEN_SEARCH_S3_SECRET_KEY`               | Object storage secret key            | Yes                     | -                 | `4756d4...b42529`              |
| `CARMEN_SEARCH_KAFKA_URI`                   | URI of the Kafka broker              | Yes                     | -                 | `kafka:9092`                   |
| `CARMEN_SEARCH_KAFKA_CONSUMER_GROUP`        | Kafka consumer group                 | For service replication | `extractor`       | -                              |
| `CARMEN_SEARCH_KAFKA_TOPIC_DOCUMENTS_QUEUE` | Documents queue topic (output)       | Yes                     | `documents.queue` | -                              |
| `CARMEN_SEARCH_KAFKA_TOPIC_CHUNKS_READY`    | Chunks ready topic (input)           | Yes                     | `chunks.queue`    | -                              |
| `CARMEN_SEARCH_SERVICE_EMBEDDING_URL`       | Base URL of the embedding service    | Yes                     | -                 | `http://embedding-search:8000` |
| `CARMEN_SEARCH_SERVICE_LANGDETECTOR_URL`    | Base URL of the langdetector service | Yes                     | -                 | `http://langdetector:8000`     |



## Example Docker Compose setup

```yaml
services:
  search:
    container_name: carmen-search
    build:
      context: ./search
    environment:
      CARMEN_SEARCH_SERVER_ADDR: :8000
      CARMEN_SEARCH_POSTGRES_DB: carmen
      CARMEN_SEARCH_POSTGRES_HOST: postgres
      CARMEN_SEARCH_POSTGRES_PORT: 5432
      CARMEN_SEARCH_POSTGRES_USER: postgres
      CARMEN_SEARCH_POSTGRES_PASSWORD: qwerty
      CARMEN_SEARCH_S3_REGION: eu-central-1
      CARMEN_SEARCH_S3_ENDPOINT: http://rustfs:9000
      CARMEN_SEARCH_S3_BUCKET: carmen-documents
      CARMEN_SEARCH_S3_ACCESS_KEY: changeme
      CARMEN_SEARCH_S3_SECRET_KEY: changeme
      CARMEN_SEARCH_KAFKA_URI: kafka-broker:9092
      CARMEN_SEARCH_KAFKA_CONSUMER_GROUP: search-consumer-group
      CARMEN_SEARCH_KAFKA_TOPIC_DOCUMENTS_QUEUE: documents.queue
      CARMEN_SEARCH_KAFKA_TOPIC_CHUNKS_READY: chunks.ready
      CARMEN_SEARCH_SERVICE_EMBEDDING_URL: http://embedding-search:8000
      CARMEN_SEARCH_SERVICE_LANGDETECTOR_URL: http://langdetector:8000
    restart: unless-stopped
    depends_on:
      kafka-broker:
        condition: service_healthy
      kafka-init:
        condition: service_completed_successfully
      postgres:
        condition: service_started
      rustfs:
        condition: service_started
      langdetector:
        condition: service_started
      embedding-search:
        condition: service_started
```
