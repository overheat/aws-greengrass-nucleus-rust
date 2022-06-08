use serde::{Deserialize, Serialize};
use tracing::{debug, event, info, span, Level};
// implements Chunkable<ComponentStatusDetails>
#[derive(Serialize, Deserialize, Debug)]
pub struct FleetStatusDetails {
    ggcVersion: String,

    platform: String,

    architecture: String,

    thing: String,

    overallStatus: OverallStatus,

    sequenceNumber: usize,

    // List<ComponentStatusDetails> componentStatusDetails,
    deploymentInformation: DeploymentInformation,
    // pub void setVariablePayload(List<ComponentStatusDetails> variablePayload) {
    //     this.setComponentStatusDetails(variablePayload),
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OverallStatus {
    HEALTHY,
    UNHEALTHY,
}

// pub const detailedStatus: &str = "detailedStatus";
// pub const failureCause: &str = "failureCause";

#[derive(Serialize, Deserialize, Debug)]
struct StatusDetails {
    pub DETAILED_STATUS_KEY: String,
    pub FAILURE_CAUSE_KEY: String,
}

// pub const STATUS_KEY: &str = "status";
// pub const STATUS_DETAILS_KEY: &str = "statusDetails";
// pub const ARN_FOR_STATUS_KEY: &str = "fleetConfigurationArnForStatus";
#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentInformation {
    status: String,
    statusDetails: StatusDetails,
    fleetConfigurationArnForStatus: String,
}

pub struct ComponentStatusDetails {
    componentName: String,

    version: String,

    // List<String> fleetConfigArns,
    statusDetails: String,

    // We need to add this since during serialization, the 'is' is removed.
    isRoot: bool,
    // State state,
}

pub const FLEET_STATUS_SERVICE_TOPICS: &str = "FleetStatusService";
pub const DEFAULT_FLEET_STATUS_SERVICE_PUBLISH_TOPIC: &str =
    "$aws/things/{thingName}/greengrassv2/health/json";
pub const FLEET_STATUS_TEST_PERIODIC_UPDATE_INTERVAL_SEC: &str = "fssPeriodicUpdateIntervalSec";
pub const DEFAULT_PERIODIC_PUBLISH_INTERVAL_SEC: usize = 86_400;
pub const FLEET_STATUS_PERIODIC_PUBLISH_INTERVAL_SEC: &str = "periodicStatusPublishIntervalSeconds";
const FLEET_STATUS_SEQUENCE_NUMBER_TOPIC: &str = "sequenceNumber";
const FLEET_STATUS_LAST_PERIODIC_UPDATE_TIME_TOPIC: &str = "lastPeriodicUpdateTime";
const MAX_PAYLOAD_LENGTH_BYTES: usize = 128_000;
pub const DEVICE_OFFLINE_MESSAGE: &str =
    "Device not configured to talk to AWS IoT cloud. FleetStatusService is offline";
// extends GreengrassService
pub struct FleetStatusService {
    // DeviceConfiguration deviceConfiguration;
    // GlobalStateChangeListener handleServiceStateChange = this::handleServiceStateChange,
    // Function<Map<String, Object>, Boolean> deploymentStatusChanged = this::deploymentStatusChanged,
    updateTopic: String,
    thingName: String,
    // MqttClient mqttClient,
    // Kernel kernel,
    architecture: String,
    platform: String,
    // MqttChunkedPayloadPublisher<ComponentStatusDetails> publisher,
    // DeploymentStatusKeeper deploymentStatusKeeper,
    //For testing

    // AtomicBoolean isConnected = new AtomicBoolean(true),
    // AtomicBoolean isEventTriggeredUpdateInProgress = new AtomicBoolean(false),
    // AtomicBoolean isFSSSetupComplete = new AtomicBoolean(false),
    // Set<GreengrassService> updatedGreengrassServiceSet =
    // Collections.newSetFromMap(new ConcurrentHashMap<>()),
    // ConcurrentHashMap<GreengrassService, Instant> serviceFssTracksMap = new ConcurrentHashMap<>(),
    // AtomicBoolean isDeploymentInProgress = new AtomicBoolean(false),
    // Object periodicUpdateInProgressLock = new Object(),
    periodicPublishIntervalSec: usize,
    // ScheduledFuture<?> periodicUpdateFuture,
}

pub const FLOW: &str = r#"Provisioning AWS IoT resources for the device with IoT Thing Name: [GreengrassQuickStartCore-new]... -> describe-endpoint
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

fn uploadFleetStatusServiceData(
    overAllStatus: OverallStatus,
    deploymentInformation: DeploymentInformation,
) {
    // if (!isConnected.get()) {
    if true {
        info!("Not updating fleet status data since MQTT connection is interrupted.");
        return;
    }
    // List<ComponentStatusDetails> components = new ArrayList<>();
    // long sequenceNumber;

    // synchronized (greengrassServiceSet)

    // FleetStatusDetails fleetStatusDetails = FleetStatusDetails.builder()
    //         .overallStatus(overAllStatus)
    //         .architecture(this.architecture)
    //         .platform(this.platform)
    //         .thing(thingName)
    //         .ggcVersion(deviceConfiguration.getNucleusVersion())
    //         .sequenceNumber(sequenceNumber)
    //         .deploymentInformation(deploymentInformation)
    //         .build();
    // logger.atInfo().event("fss-status-update-published").log("fleetStatusDetails {} components {}",
    //         fleetStatusDetails, components);

    // publisher.publish(fleetStatusDetails, components);
    info!(
        event = "fss-status-update-published",
        "Status update published to FSS"
    );
}