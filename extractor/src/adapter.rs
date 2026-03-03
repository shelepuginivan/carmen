use anyhow::bail;
use log::{error, info};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::FutureProducer;
use rdkafka::{ClientConfig, Message};
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::{select, signal};

use crate::config::Config;
use crate::models::Document;

pub struct DocumentAdapter {
    producer: FutureProducer,
    consumer: StreamConsumer,
    bucket: Box<Bucket>,
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

        let region = Region::Custom {
            region: cfg.s3_region.clone(),
            endpoint: cfg.s3_endpoint.clone(),
        };

        let credentials = Credentials::new(
            Some(&cfg.s3_access_key),
            Some(&cfg.s3_secret_key),
            None,
            None,
            None,
        )?;

        let bucket = Bucket::new(&cfg.s3_bucket, region, credentials)?.with_path_style();

        consumer.subscribe(&[&cfg.kafka_topic_documents_queue])?;

        Ok(Self {
            producer,
            consumer,
            bucket,
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

        let document_bytes = self
            .bucket
            .get_object(document.object_key)
            .await?
            .as_slice();

        Ok(())
    }
}
