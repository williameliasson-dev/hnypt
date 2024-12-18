use k8s_openapi::api::core::v1::Service;
use kube::api::PostParams;
use kube::{Api, Client, ResourceExt};

use crate::logger::Logger;

use super::{get_service_from_spec, ServicesTypes};

async fn setup_service(
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
        Err(kube::Error::Api(kube_error)) => {
            // Syntax is understandable but failed to setup..
            if kube_error.reason == "Invalid" && kube_error.code == 422 {
                Logger::warn(
                    format!("Service failed to setup but syntax is correct: {}", name).as_str(),
                );
            } else {
                return Err(kube_error.into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

pub async fn setup_services() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let services_api: Api<Service> = Api::default_namespaced(client);

    let services_to_setup: [&ServicesTypes; 2] =
        [&ServicesTypes::MONGODB, &ServicesTypes::RABBITMQ];

    for service in services_to_setup.iter() {
        let service_spec = get_service_from_spec(service).unwrap();
        setup_service(service_spec, &services_api).await?;
    }

    Ok(())
}
