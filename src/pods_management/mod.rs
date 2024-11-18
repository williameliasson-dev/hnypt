mod pods_enums;
mod pods_setup;
mod pods_utils;

pub use pods_enums::PodTypes;
pub use pods_setup::setup_pods;
pub use pods_utils::{assure_pod_is_running, get_pod_from_spec};

pub struct Pods;

impl Pods {
    pub async fn init() -> anyhow::Result<()> {
        setup_pods().await?;

        Ok(())
    }
}
