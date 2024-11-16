use k8s_openapi::api::core::v1::Service;
use kube::api::PostParams;
use kube::{Api, ResourceExt};

use crate::logger::Logger;

pub async fn setup_service(
    service_to_setup: Service,
    services_api: &Api<Service>,
) -> anyhow::Result<()> {
    let post_params = PostParams::default();
    let name = service_to_setup.name_any();

    match services_api.create(&post_params, &service_to_setup).await {
        Ok(..) => {
            assert_eq!(name, service_to_setup.name_any());
            Logger::info(format!("Service has been setup: {}", name).as_str());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
