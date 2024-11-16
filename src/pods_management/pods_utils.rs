use k8s_openapi::api::core::v1::Pod;
use kube::runtime::conditions::is_pod_running;
use kube::runtime::wait::await_condition;
use kube::Api;
use tokio::time;

use crate::logger::Logger;

pub enum PodTypes {
    MONGODB,
    HONEYPOT,
}

pub fn get_pod_from_spec(pod_type: &PodTypes) -> anyhow::Result<Pod> {
    let spec_content: &str = match pod_type {
        PodTypes::MONGODB => include_str!("../pods/mongodb.json"),
        PodTypes::HONEYPOT => include_str!("../pods/honeypot.json"),
    };

    let honey_pot_pod: Pod =
        serde_json::from_str(&spec_content).expect("Failed to read JSON from pod spec");

    return Ok(honey_pot_pod);
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