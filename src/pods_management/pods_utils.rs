use crate::logger::Logger;
use k8s_openapi::api::core::v1::Pod;
use kube::runtime::conditions::is_pod_running;
use kube::runtime::wait::await_condition;
use kube::Api;
use tokio::time;

use super::PodTypes;

pub fn get_pod_from_spec(pod_type: &PodTypes) -> anyhow::Result<Pod> {
    let spec_content: &str = match pod_type {
        PodTypes::MONGODB => include_str!("../manifests/pods/mongodb.yaml"),
        PodTypes::HONEYPOT => include_str!("../manifests/pods/honeypot.yaml"),
        PodTypes::RABBITMQ => include_str!("../manifests/pods/rabbitmq.yaml"),
    };

    let pod: Pod = serde_yml::from_str(&spec_content).expect("Failed to read JSON from pod spec");

    return Ok(pod);
}

pub async fn assure_pod_is_running(pod_name: &str, pods_api: &Api<Pod>) -> anyhow::Result<()> {
    const ASSURE_CONNECTION_TIMEOUT: u64 = 120;

    Logger::info(format!("Trying to establish connection with: {}", pod_name).as_str());

    let establish = await_condition(pods_api.clone(), pod_name, is_pod_running());

    match tokio::time::timeout(
        time::Duration::from_secs(ASSURE_CONNECTION_TIMEOUT),
        establish,
    )
    .await?
    {
        Ok(..) => Logger::info("Connection to pod established"),
        Err(..) => {
            Logger::error(format!("Couldn't establish connection to pod: {}", pod_name).as_str())
        }
    }

    Ok(())
}
