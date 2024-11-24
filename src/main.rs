pub mod db;
use db::MongoDB;

pub mod logger;

mod pods_management;
use pods_management::Pods;

use services_management::Services;
mod services_management;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Pods::init().await?;
    Services::init().await?;
    MongoDB::init().await?;

    Ok(())
}
