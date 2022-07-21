pub mod deployment;
pub mod kernel;
pub mod main;
pub mod policy;
pub mod status;
pub mod telemetry;

use crate::dependency::State;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use dashmap::DashMap;
use once_cell::sync::Lazy;

pub static SERVICES: Lazy<DashMap<String, ServiceStatus>> = Lazy::new(|| DashMap::new());

pub trait Service {
    fn new(name: &'static str, ver: &'static str) -> ServiceStatus {
        ServiceStatus {
            componentName: name,
            version: ver,
            fleetConfigArns: vec![],
            statusDetails: json!(null),
            isRoot: false,
            status: State::FINISHED,
        }
    }
    fn enable();
    // fn disable() -> bool;
    // fn start() {}
    // fn restart() -> bool;
    // fn stop() -> bool;
    // fn status() -> State {}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceStatus {
    componentName: &'static str,
    version: &'static str,
    fleetConfigArns: Vec<String>,
    statusDetails: Value,
    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    status: State,
}

use deployment::Deployments;
use kernel::Kernel;
use main::Main;
use policy::Policy;
use status::Status;
use telemetry::Telemetry;

pub fn start_services() {
    Kernel::enable();
    Main::enable();
    Policy::enable();
    Deployments::enable();
    Telemetry::enable();
    Status::enable();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    use crate::services::{Service, SERVICES};

    use super::deployment::Deployments;
    use super::kernel::Kernel;
    use super::policy::Policy;
    use super::telemetry::Telemetry;
    use crate::dependency::State;

    #[test]
    fn services_test() {
        Kernel::enable();
        assert_eq!(
            SERVICES
                .get("aws.greengrass.Nucleus")
                .unwrap()
                .componentName,
            ""
        );
        assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().version, "");
        // assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().fleetConfigArns, vec![]);
        // assert_eq!(SERVICES.get("aws.greengrass.Nucleus").unwrap().statusDetails, null);
        assert_eq!(
            SERVICES.get("aws.greengrass.Nucleus").unwrap().isRoot,
            false
        );
        assert_eq!(
            SERVICES.get("aws.greengrass.Nucleus").unwrap().status,
            State::FINISHED
        );
    }
}