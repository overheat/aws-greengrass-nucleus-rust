pub mod greengrassv2;
pub mod iot;

use std::path::Path;

use anyhow::Ok;
use aws_config::meta::region::RegionProviderChain;
use aws_types::region::Region;
use clap::Parser;
use greengrassv2 as ggv2;
use tracing::{debug, event, info, span, Level};

pub struct SystemConfiguration {
    certificateFilePath: String,
    privateKeyPath: String,
    rootCAPath: String,
    thingName: String,
}

pub struct NucleusConfiguration {
    awsRegion: String,
    iotCredentialsEndpoint: String,
    iotDataEndpoint: String,
    iotRoleAlias: String,
}

pub struct ProvisionConfiguration {
    systemConfiguration: SystemConfiguration,
    nucleusConfiguration: NucleusConfiguration,
}

// pub class ProvisionContext {
//     String provisioningPolicy;
//     Map<String, Object> parameterMap;
// }

const FLOW: &str = r#"Provisioning AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]... -> describe-endpoint
                        Found IoT policy "GreengrassV2IoTThingPolicy", reusing it -> list-policies | create-policy & get-policy
                        Creating keys and certificate... -> create-keys-and-certificate
                        Attaching policy to certificate...  -> attach-policy
                        Creating IoT Thing "GreengrassQuickStartCore-new"... -> create-thing
                        Attaching certificate to IoT thing... -> attach-thing-principal
                        Successfully provisioned AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]!
                        Setting up resources for aws.greengrass.TokenExchangeService ... -> IAM create-role
                        Attaching TES role policy to IoT thing... -> IAM attach-role-policy
                        No managed IAM policy found, looking for user defined policy... -> IAM create-policy
                        IAM policy named "GreengrassV2TokenExchangeRoleAccess" already exists. Please attach it to the IAM role if not already
                        Configuring Nucleus with provisioned resource details... -> create-role-alias
                        Root CA file found at "/greengrass/v2/rootCA.pem". Contents will be preserved. -> reqwest crate.
                        Downloading Root CA from "https://www.amazontrust.com/repository/AmazonRootCA1.pem"
                        Created device configuration -> config file.
                        Successfully configured Nucleus with provisioned resource details!
                        Launching Nucleus... -> mqtt publish.
                        Launched Nucleus successfully.
                        "#;

pub fn print_flow() {
    info!("{}", FLOW);
}

pub async fn init(region: RegionProviderChain) {
    let shared_config = aws_config::from_env().region(region).load().await;

    ggv2::ggv2_init(&shared_config).await;
}

/**
 * Updates the system configuration values in kernel config as per the given {@link SystemConfiguration}.
 * @param systemConfiguration {@link SystemConfiguration}
 * @param updateBehavior Update behavior indicating either merge or replace
 */
pub fn updateSystemConfiguration() {}
/**
 * Updates the nucleus configuration value in kernel config as per the given {@link NucleusConfiguration}.
 * @param nucleusConfiguration {@link NucleusConfiguration}
 * @param updateBehavior Update behavior indicating either merge or replace
 */
pub fn updateNucleusConfiguration() {}
