use dotenv::dotenv;
use futures_lite::stream::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use once_cell::sync::OnceCell;
use std::env;
pub struct RabbitMqClient;

static CHANNEL: OnceCell<Channel> = OnceCell::new();

impl RabbitMqClient {
    pub async fn init() -> anyhow::Result<()> {
        dotenv().ok();

        let uri = match env::var("RABBITMQ_URI") {
            Ok(uri) => uri,
            Err(_) => panic!("MONGODB_URI not set"),
        };

        let conn = Connection::connect(uri.as_str(), ConnectionProperties::default()).await?;

        let channel = conn.create_channel().await?;

        CHANNEL.set(channel).expect("");
        Ok(())
    }

    fn get_channel() -> anyhow::Result<&'static Channel> {
        CHANNEL
            .get()
            .ok_or_else(|| anyhow::anyhow!("Channel not initialized"))
    }

    pub async fn consume(&self, queue: &str) -> anyhow::Result<()> {
        let channel = Self::get_channel()?;

        let mut consumer = channel
            .basic_consume(
                queue,
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("er");
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }

        Ok(())
    }

    pub async fn publish(&self, queue: &str, payload: &[u8]) -> anyhow::Result<()> {
        let channel = Self::get_channel()?;

        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }
}
