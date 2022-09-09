use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::{Error, Result};
use aws_iot_device_sdk::shadow;
use aws_sdk_greengrassv2::{Client, Region};
use bytes::Bytes;
use once_cell::sync::Lazy;
use rumqttc::Publish;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio::time;

use crate::services::{Service, SERVICES};

// const long TIMEOUT_FOR_SUBSCRIBING_TO_TOPICS_SECONDS = Duration.ofMinutes(1).getSeconds();
// const long TIMEOUT_FOR_PUBLISHING_TO_TOPICS_SECONDS = Duration.ofMinutes(1).getSeconds();
// const long WAIT_TIME_TO_SUBSCRIBE_AGAIN_IN_MS = Duration.ofMinutes(2).toMillis();
// const Logger logger = LogManager.getLogger(ShadowDeploymentListener.class);
pub const CONFIGURATION_ARN_LOG_KEY_NAME: &str = "CONFIGURATION_ARN";
pub const DESIRED_STATUS_KEY: &str = "desiredStatus";
pub const FLEET_CONFIG_KEY: &str = "fleetConfig";
pub const GGC_VERSION_KEY: &str = "ggcVersion";
pub const DESIRED_STATUS_CANCELED: &str = "CANCELED";
pub const DEPLOYMENT_SHADOW_NAME: &str = "AWSManagedGreengrassV2Deployment";
pub const DEVICE_OFFLINE_MESSAGE: &str = "Device not configured to talk to AWS Iot cloud. ";
// + "Single device deployment is offline";
pub const SUBSCRIBING_TO_SHADOW_TOPICS_MESSAGE: &str = "Subscribing to Iot Shadow topics";

const VERSION: &str = "2.5.6";
const NAME: &str = "DeploymentService";
pub struct Deployments {}

impl Service for Deployments {
    fn enable() {
        SERVICES.insert(NAME.into(), Self::new(NAME, VERSION));
    }
}

static DEPLOYSTATUS: DeployStates = DeployStates {
    mutex: Mutex::new(States::Deployment),
};

#[derive(Debug, Copy, Clone)]
enum States {
    Deployment = 0,
    Inprogress,
    Succeed,
}
struct DeployStates {
    mutex: Mutex<States>,
}
impl DeployStates {
    fn get(&self) -> States {
        let lock = self.mutex.lock().unwrap();
        *lock
    }
    fn reset(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock = States::Deployment;
    }
    fn next(&self) {
        let mut lock = self.mutex.lock().unwrap();
        match *lock {
            States::Deployment => *lock = States::Inprogress,
            States::Inprogress => *lock = States::Succeed,
            States::Succeed => self.reset(),
        }
    }
}

pub async fn connect_shadow(mqtt_client: &AsyncClient, thing_name: &str) {
    let topic = format!("$aws/things/{thing_name}/shadow/name/{DEPLOYMENT_SHADOW_NAME}/#");
    mqtt_client
        .subscribe(&topic, QoS::AtMostOnce)
        .await
        .unwrap();
}

pub async fn disconnect_shadow(mqtt_client: AsyncClient, thing_name: &str) {
    let topic = format!("$aws/things/{thing_name}/shadow/name/{DEPLOYMENT_SHADOW_NAME}/#");
    mqtt_client.unsubscribe(&topic).await.unwrap();
}

fn assemble_payload(thing_name: &str, arn: &str, version: &str, next: bool) -> Value {
    let version: u8 = version.parse().unwrap();
    if next {
        json!({
          "shadowName": DEPLOYMENT_SHADOW_NAME,
          "thingName": thing_name,
          "state": {
            "reported": {
              "ggcVersion": VERSION,
              "fleetConfigurationArnForStatus": arn,
              "statusDetails": {},
              "status": "IN_PROGRESS"
            }
          },
          "version": version
        })
    } else {
        json!({
          "shadowName": DEPLOYMENT_SHADOW_NAME,
          "thingName": thing_name,
          "state": {
            "reported": {
              "ggcVersion": VERSION,
              "fleetConfigurationArnForStatus": arn,
              "statusDetails": {
                    "detailedStatus": "SUCCESSFUL"
              },
              "status": "SUCCEEDED"
            }
          },
          "version": version
        })
    }
}

pub async fn resp_shadow_delta(v: Publish, tx: Sender<Publish>) {
    let v: Value = serde_json::from_slice(&v.payload).unwrap();
    let shadow_version = v["version"].to_string();
    let v = v["state"]["fleetConfig"]
        .to_string()
        .trim_matches('"')
        .replace("\\", "");
    let v: Value = serde_json::from_str(&v).unwrap();

    // "arn:aws:greengrass:<region>:<id>:configuration:thing/<name>:<version>"
    let configuration_arn = v["configurationArn"].as_str().unwrap();
    let (other, version) = configuration_arn.rsplit_once(':').unwrap();
    let (_, thing_name) = other.rsplit_once('/').unwrap();

    let topic = shadow::assemble_topic(
        shadow::Topic::Update,
        thing_name,
        Some(DEPLOYMENT_SHADOW_NAME),
    )
    .unwrap();
    let components = v["components"].to_string();
    let (components_name, components_version) =
        v["components"].to_string().rsplit_once(':').unwrap();

    match DEPLOYSTATUS.get() {
        States::Deployment => {
            let payload =
                assemble_payload(thing_name, configuration_arn, &shadow_version, true).to_string();
            let value = Publish {
                dup: false,
                qos: QoS::AtMostOnce,
                retain: false,
                pkid: 0,
                topic: topic.to_string(),
                payload: Bytes::from(payload),
            };
            tx.send(value).await;
            let mut map: HashMap<String, HashMap<String, serde_json::Value>> =
                serde_json::from_value(v["components"].to_owned()).unwrap();
            for (k, v) in map.drain().take(1) {
                component_deploy(k, v.get("version").unwrap().to_string()).await;
            }
            // time::sleep(Duration::from_secs(3)).await;
        }
        States::Inprogress => {
            let payload = assemble_payload(thing_name, &configuration_arn, &shadow_version, false)
                .to_string();
            let value = Publish {
                dup: false,
                qos: QoS::AtMostOnce,
                retain: false,
                pkid: 0,
                topic: topic.to_string(),
                payload: Bytes::from(payload),
            };
            tx.send(value).await;
        }
        States::Succeed => {}
    }
    DEPLOYSTATUS.next();
}

async fn component_deploy(name: String, version: String) {
    println!("{}:{}", name, version);

    // 1. resolve-component-candidates (option)
    // 2. get-component to get recipe.
    // 3. get-s3 for private component.

}
