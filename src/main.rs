use std::env;

pub mod db;

use anyhow::Ok;
use db::MongoDB;

pub mod logger;

mod pods_management;
use pods_management::Pods;

use rabbitmq::RabbitMqClient;
use services_management::Services;
mod services_management;

pub mod rabbitmq;

async fn setup() -> anyhow::Result<()> {
    Pods::init().await?;
    Services::init().await?;

    Ok(())
}

async fn start() -> anyhow::Result<()> {
    RabbitMqClient::init().await?;
    MongoDB::init().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let arg = args[1].as_str();

        match arg {
            "setup" => setup().await?,
            "start" => start().await?,
            _ => println!("Unknown arguments"),
        }
    };

    Ok(())
}
