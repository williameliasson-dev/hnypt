use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::{Api, Client, ResourceExt};

use crate::logger::Logger;

use super::{assure_pod_is_running, get_pod_from_spec, PodTypes};

async fn setup_pod(pod_to_setup: Pod, pods_api: &Api<Pod>) -> anyhow::Result<()> {
    let post_params = PostParams::default();
    let name = pod_to_setup.name_any();

    match pods_api.create(&post_params, &pod_to_setup).await {
        Ok(..) => {
            assert_eq!(name, pod_to_setup.name_any());
            Logger::info(format!("Pod has been setup: {}", name).as_str());
            assure_pod_is_running(&name, pods_api).await?;
        }
        Err(kube::Error::Api(kube_error)) => {
            // If we want to handle more reasons like this setup an enum.
            if kube_error.reason == "AlreadyExists" {
                Logger::warn(format!("Pod already exists: {}", name).as_str());
            } else {
                return Err(kube_error.into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

pub async fn setup_pods() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let pods_api: Api<Pod> = Api::default_namespaced(client);

    let pods_to_setup: [&PodTypes; 3] =
        [&PodTypes::MONGODB, &PodTypes::HONEYPOT, &PodTypes::RABBITMQ];

    for pod in pods_to_setup.iter() {
        let pod_spec = get_pod_from_spec(pod).unwrap();
        setup_pod(pod_spec, &pods_api).await?;
    }

    Ok(())
}
