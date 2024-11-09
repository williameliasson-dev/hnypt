use k8s_openapi::api::core::v1::Pod;
use kube::runtime::conditions::is_pod_running;
use kube::runtime::wait::await_condition;
use kube::Api;
use std::fs::{self};
use tokio::time;

pub fn get_pod_from_spec(path: &str) -> anyhow::Result<Pod> {
    let file_content_as_string = fs::read_to_string(path)?;

    let honey_pot_pod: Pod =
        serde_json::from_str(&file_content_as_string).expect("Failed to read JSON from pod spec");

    return Ok(honey_pot_pod);
}

pub async fn assure_pod_is_running(pod_name: &str, pods_api: &Api<Pod>) -> anyhow::Result<()> {
    const ASSURE_CONNECTION_TIMEOUT: u64 = 15;

    let establish = await_condition(pods_api.clone(), pod_name, is_pod_running());

    match tokio::time::timeout(
        time::Duration::from_secs(ASSURE_CONNECTION_TIMEOUT),
        establish,
    )
    .await?
    {
        Ok(..) => println!("Connection to pod confirmed"),
        Err(..) => println!("Couldn't establish connection to pod: {}", pod_name),
    }

    Ok(())
}
