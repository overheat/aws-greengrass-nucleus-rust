use crate::services::{Service, SERVICES};

const VERSION: &str = "2.5.5";
pub struct Kernel {}

impl Service for Kernel {
    fn enable() -> bool {
        SERVICES.insert("aws.greengrass.Nucleus".to_string(), 0);
        true
    }
}
