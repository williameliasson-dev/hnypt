use k8s_openapi::api::core::v1::Service;

use super::ServicesTypes;

pub fn get_service_from_spec(service_type: &ServicesTypes) -> anyhow::Result<Service> {
    let spec_content: &str = match service_type {
        ServicesTypes::MONGODB => include_str!("../services/mongodb-service.json"),
    };

    let service: Service =
        serde_json::from_str(&spec_content).expect("Failed to read JSON from service spec");

    return Ok(service);
}
