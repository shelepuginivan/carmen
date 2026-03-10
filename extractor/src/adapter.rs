use anyhow::bail;
use futures::future::try_join_all;
use log::{error, info};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};
use std::time::Duration;
use tokio::{select, signal};

use crate::config::Config;
use crate::models::{Chunk, Document};
use crate::processor::Processor;
use crate::storage::DocumentStorage;

pub struct DocumentAdapter {
    producer: FutureProducer,
    consumer: StreamConsumer,
    storage: DocumentStorage,
    processor: Processor,
    chunks_queue: String,
}

impl DocumentAdapter {
    pub fn new(
        cfg: &Config,
        storage: DocumentStorage,
        processor: Processor,
    ) -> anyhow::Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.kafka_uri)
            .set("queue.buffering.max.ms", "0")
            .create()?;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.kafka_uri)
            .set("group.id", &cfg.kafka_consumer_group)
            .create()?;

        consumer.subscribe(&[&cfg.kafka_topic_documents_queue])?;

        Ok(Self {
            producer,
            consumer,
            storage,
            processor,
            chunks_queue: cfg.kafka_topic_chunks_queue.clone(),
        })
    }

    pub async fn handle(&self) {
        loop {
            select! {
                message = self.consumer.recv() => {
                    let msg = match message {
                        Ok(m) => m,
                        Err(e) => {
                            error!("Failed to receive message: {e}");
                            continue;
                        }
                    };

                    if let Err(e) = self.handle_message(msg).await {
                        error!("Failed to process message: {e}");
                    }
                },
                _ = signal::ctrl_c() => {
                    info!("Received SIGINT, stopping document adapter...");
                    break;
                }
            }
        }

        self.consumer.unsubscribe();
    }

    async fn handle_message(&self, msg: BorrowedMessage<'_>) -> anyhow::Result<()> {
        let payload = match msg.payload() {
            Some(p) => p,
            None => bail!("failed to retrieve message payload"),
        };

        let document: Document = serde_json::from_slice(payload)?;
        info!("Processing document {}...", document.id);

        let document_bytes = self.storage.get_document(&document.object_key).await?;
        let chunk_iterator = self.processor.process(&document.id, document_bytes)?;

        let handles = chunk_iterator.map(|text| {
            let chunk = Chunk {
                id: &document.id,
                text,
            };

            self.send_chunk(chunk)
        });

        try_join_all(handles).await?;

        Ok(())
    }

    async fn send_chunk(&self, chunk: Chunk<'_>) -> anyhow::Result<()> {
        let msg = serde_json::to_string(&chunk).expect("should serialize chunk");

        self.producer
            .send(
                FutureRecord::<(), _>::to(&self.chunks_queue).payload(&msg),
                Duration::from_secs(1),
            )
            .await
            .map_err(|e| e.0)?;

        Ok(())
    }
}
