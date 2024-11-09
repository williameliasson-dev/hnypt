use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
mod pods_management;

use pods_management::{get_pod_from_spec, setup_pod};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let honey_pot_pod = get_pod_from_spec("src/pods/honeypot.json").unwrap();

    let client = Client::try_default().await?;
    let pods_api: Api<Pod> = Api::default_namespaced(client);
    setup_pod(honey_pot_pod, &pods_api).await?;

    Ok(())
}
