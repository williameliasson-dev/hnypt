use k8s_openapi::api::core::v1::Service;

use super::ServicesTypes;

pub fn get_service_from_spec(service_type: &ServicesTypes) -> anyhow::Result<Service> {
    let spec_content: &str = match service_type {
        ServicesTypes::MONGODB => include_str!("../manifests/services/mongodb-nodeport.yaml"),
        ServicesTypes::RABBITMQ => include_str!("../manifests/services/rabbitmq-nodeport.yaml"),
    };

    let service: Service =
        serde_yml::from_str(&spec_content).expect("Failed to read JSON from service spec");

    return Ok(service);
}
