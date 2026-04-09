# embedding

Service `embedding` generates embeddings. It operates in 2 modes:
1. Generates embeddings for document chunks and sends them to the search service
2. Generates embeddings for search queries via REST API

## Environment variables

| Variable                                      | Description                       | Required                         | Default        | Example                    |
| --------------------------------------------- | --------------------------------- | -------------------------------- | -------------- | -------------------------- |
| `CARMEN_EMBEDDING_WORKING_MODE`               | Working mode of the service       | Yes                              | -              | `chunks` or `search`       |
| `CARMEN_EMBEDDING_SENTENCE_TRANSFORMERS_HOME` | Path for local SBERT models       | No (see notes below)             | -              | `/opt/models`              |
| `CARMEN_EMBEDDING_MODEL__*`                   | SBERT model for specific language | Yes                              | -              | _See notes below_          |
| `CARMEN_EMBEDDING_LANGDETECTOR_URL`           | URL of the `langdetector` service | Yes                              | -              | `http://langdetector:8000` |
| `CARMEN_EMBEDDING_KAFKA_URI`                  | URI of the Kafka broker           | For `chunks` working mode        | -              | `kafka:9092`               |
| `CARMEN_EMBEDDING_KAFKA_CONSUMER_GROUP`       | Kafka consumer group              | For `chunks` service replication | `extractor`    | -                          |
| `CARMEN_EMBEDDING_KAFKA_TOPIC_CHUNKS_QUEUE`   | Chunks queue topic (input)        | For `chunks`                     | `chunks.queue` | -                          |
| `CARMEN_EMBEDDING_KAFKA_TOPIC_CHUNKS_READY`   | Topic for ready chunks (output)   | For `chunks`                     | `chunks.ready` | -                          |

**Notes:**
1.  You can configure building of the OCI image to pre-download SBERT models.
    Provide `SENTENCE_TRANSFORMERS_HOME` and `SENTENCE_TRANSFORMERS_MODELS`
    build args, for example:
    ```yaml
    # ...
      build:
        context: ./embedding
        args:
          SENTENCE_TRANSFORMERS_HOME: /opt/sentence-transformers
          SENTENCE_TRANSFORMERS_MODELS: |
            [
              "sentence-transformers/static-similarity-mrl-multilingual-v1"
            ]
    ```
    Runtime environment variables should match these build args. If
    `SENTENCE_TRANSFORMERS_HOME` is set, the service will only use local
    models.
2.  Configure env variables `CARMEN_EMBEDDING_MODEL__*` for each language you
    want to support, e.g. `CARMEN_EMBEDDING_MODEL__ENGLISH` or
    `CARMEN_EMBEDDING_MODEL__EN`. You should provide models for at least every
    language that can be detected with [langdetector service](../langdetector/README.md).
    
## Example Docker Compose setup

1. `chunks` mode

```yaml
services:
  embedding-chunks:
    image: localhost/carmen_embedding:latest
    container_name: carmen-embedding-chunks
    build:
      context: ./embedding
      args:
        # Should match environment variables below.
        SENTENCE_TRANSFORMERS_HOME: /opt/sentence-transformers
        SENTENCE_TRANSFORMERS_MODELS: |
          [
            "sentence-transformers/static-similarity-mrl-multilingual-v1"
          ]
    environment:
      CARMEN_EMBEDDING_WORKING_MODE: chunks
      CARMEN_EMBEDDING_SENTENCE_TRANSFORMERS_HOME: /opt/sentence-transformers
      CARMEN_EMBEDDING_MODEL__ENGLISH: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_MODEL__RUSSIAN: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_MODEL__SPANISH: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_LANGDETECTOR_URL: http://langdetector:8000
      CARMEN_EMBEDDING_KAFKA_URI: kafka-broker:9092
      CARMEN_EMBEDDING_KAFKA_CONSUMER_GROUP: embedding-consumer-group
      CARMEN_EMBEDDING_KAFKA_TOPIC_CHUNKS_QUEUE: chunks.queue
      CARMEN_EMBEDDING_KAFKA_TOPIC_CHUNKS_READY: chunks.ready
    restart: unless-stopped
    depends_on:
      kafka-init:
        condition: service_completed_successfully
      kafka-broker:
        condition: service_healthy
      langdetector:
        condition: service_started
```

2. `search` mode

```yaml
services:
  embedding-search:
    # Omit the `build` section here, and provide matching `image` field.
    # Otherwise, the same image will be built twice.
    image: localhost/carmen_embedding:latest
    container_name: carmen-embedding-search
    environment:
      CARMEN_EMBEDDING_WORKING_MODE: search
      CARMEN_EMBEDDING_SENTENCE_TRANSFORMERS_HOME: /opt/sentence-transformers
      CARMEN_EMBEDDING_MODEL__ENGLISH: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_MODEL__RUSSIAN: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_MODEL__SPANISH: sentence-transformers/static-similarity-mrl-multilingual-v1
      CARMEN_EMBEDDING_LANGDETECTOR_URL: http://langdetector:8000
    restart: unless-stopped
    depends_on:
      langdetector:
        condition: service_started
```
