mod services_enums;
mod services_setup;
mod services_utils;

pub use services_enums::ServicesTypes;
pub use services_setup::setup_services;
pub use services_utils::get_service_from_spec;

pub struct Services;

impl Services {
    pub async fn init() -> anyhow::Result<()> {
        setup_services().await?;

        Ok(())
    }
}
