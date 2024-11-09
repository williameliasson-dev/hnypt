use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::runtime::conditions::is_pod_running;
use kube::runtime::wait::await_condition;
use kube::{Api, Client, ResourceExt};
use std::fs::{self};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //TODO: Move into mod.
    async fn setup_pod(pod_to_setup: Pod, pods_api: &Api<Pod>) -> anyhow::Result<()> {
        let post_params = PostParams::default();
        match pods_api.create(&post_params, &pod_to_setup).await {
            Ok(pod) => {
                let name = pod.name_any();
                assert_eq!(name, pod_to_setup.name_any());
                println!("Created pod: {}", name);
                assure_pod_is_running(&name, pods_api).await?;
            }
            Err(kube::Error::Api(kube_error)) => {
                assert_eq!(kube_error.code, 409);
                println!("Pod already exists");
            }
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }
    //TODO: Move into mod
    fn get_pod_from_spec(path: &str) -> anyhow::Result<Pod> {
        let file_content_as_string = fs::read_to_string(path)?;

        let honey_pot_pod: Pod = serde_json::from_str(&file_content_as_string)
            .expect("Failed to read JSON from pod spec");

        return Ok(honey_pot_pod);
    }

    //TODO: Move into mod
    async fn assure_pod_is_running(pod_name: &str, pods_api: &Api<Pod>) -> anyhow::Result<()> {
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

    let honey_pot_pod = get_pod_from_spec("src/pods/honeypot.json").unwrap();

    let client = Client::try_default().await?;
    let pods_api: Api<Pod> = Api::default_namespaced(client);
    setup_pod(honey_pot_pod, &pods_api).await?;

    Ok(())
}
