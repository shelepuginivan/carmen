use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::FutureProducer;
use rdkafka::{ClientConfig, Message};

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
            .set("consumer.group", &cfg.kafka_consumer_group)
            .create()?;

        consumer.subscribe(&[&cfg.kafka_topic_documents_queue])?;

        Ok(Self { producer, consumer })
    }
}
