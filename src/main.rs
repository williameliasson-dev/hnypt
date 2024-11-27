pub mod db;
use db::MongoDB;

pub mod logger;

mod pods_management;
use pods_management::Pods;

use rabbitmq::RabbitMqClient;
use services_management::Services;
mod services_management;

pub mod rabbitmq;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Pods::init().await?;
    Services::init().await?;
    RabbitMqClient::init().await?;
    MongoDB::init().await?;

    Ok(())
}
