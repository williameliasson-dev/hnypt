use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};

pub mod db;
pub mod logger;

mod pods_management;
use pods_management::{get_pod_from_spec, setup_pod, PodTypes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods_api: Api<Pod> = Api::default_namespaced(client);

    let pods_to_setup: [&PodTypes; 2] = [&PodTypes::MONGODB, &PodTypes::HONEYPOT];

    for pod in pods_to_setup.iter() {
        let pod_spec = get_pod_from_spec(pod).unwrap();
        setup_pod(pod_spec, &pods_api).await?;
    }

    Ok(())
}
