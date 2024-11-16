use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::{Api, ResourceExt};

use crate::logger::Logger;

use super::assure_pod_is_running;

pub async fn setup_pod(pod_to_setup: Pod, pods_api: &Api<Pod>) -> anyhow::Result<()> {
    let post_params = PostParams::default();
    let name = pod_to_setup.name_any();

    match pods_api.create(&post_params, &pod_to_setup).await {
        Ok(..) => {
            assert_eq!(name, pod_to_setup.name_any());
            Logger::info(format!("Pod has been setup: {}", name).as_str());
            assure_pod_is_running(&name, pods_api).await?;
        }
        Err(kube::Error::Api(kube_error)) => {
            assert_eq!(kube_error.code, 409);
            Logger::warn(format!("Pod already exists: {}", name).as_str());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
