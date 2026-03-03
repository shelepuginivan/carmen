use std::io::Write;

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::FutureProducer;
use rdkafka::{ClientConfig, Message};
use tokio::{select, signal};

use crate::config::Config;

pub struct DocumentAdapter {
    producer: FutureProducer,
    consumer: StreamConsumer,
}

impl DocumentAdapter {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.kafka_uri)
            .set("queue.buffering.max.ms", "0")
            .create()?;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.kafka_uri)
            .set("group.id", &cfg.kafka_consumer_group)
            .create()?;

        consumer.subscribe(&[&cfg.kafka_topic_documents_queue])?;

        Ok(Self { producer, consumer })
    }

    pub async fn handle(&self) {
        loop {
            select! {
                message = self.consumer.recv() => {
                    if let Ok(msg) = message {
                        self.handle_message(msg).await;
                        continue;
                    }
                },
                _ = signal::ctrl_c() => {
                    println!("Received SIGING, shutting down gracefully...");
                    break;
                }
            }
        }

        self.consumer.unsubscribe();
    }

    async fn handle_message(&self, msg: BorrowedMessage<'_>) {
        let payload = match msg.payload() {
            Some(p) => p,
            None => return,
        };

        let _ = std::io::stderr().write(payload);
    }
}
