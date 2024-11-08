use k8s_openapi::api::core::v1::Node;
use kube::{Api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let nodes: Api<Node> = Api::all(client);
    let node_count = nodes.list(&Default::default()).await?.items.len();

    println!("Number of nodes: {}", { node_count });

    Ok(())
}
