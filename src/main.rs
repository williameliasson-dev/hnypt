use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
mod pods_management;

use pods_management::{get_pod_from_spec, setup_pod, PodTypes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods_api: Api<Pod> = Api::default_namespaced(client);

    let mongodb_pod: Pod = get_pod_from_spec(&PodTypes::MONGODB).unwrap();
    let honey_pot_pod: Pod = get_pod_from_spec(&PodTypes::HONEYPOT).unwrap();
    setup_pod(honey_pot_pod, &pods_api).await?;
    setup_pod(mongodb_pod, &pods_api).await?;
    Ok(())
}
