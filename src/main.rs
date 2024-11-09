use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::{Api, Client, ResourceExt};
use std::fs::{self};

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
            }
            Err(kube::Error::Api(kube_error)) => {
                assert_eq!(kube_error.code, 409);
                println!("Pod already exists");
            }
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }

    fn get_pod_from_spec(path: String) -> anyhow::Result<Pod> {
        let file_content_as_string = fs::read_to_string(path)?;

        let honey_pot_pod: Pod = serde_json::from_str(&file_content_as_string)
            .expect("Failed to read JSON from pod spec");

        return Ok(honey_pot_pod);
    }
    let honey_pot_pod_spec = "src/pods/honeypot.json".to_string();
    let honey_pot_pod = get_pod_from_spec(honey_pot_pod_spec).unwrap();

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    setup_pod(honey_pot_pod, &pods).await?;

    Ok(())
}
