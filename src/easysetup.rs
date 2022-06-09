use super::provisioning;
use anyhow::Result;
use aws_sdk_iot::Client;
use std::fs;
use std::path::Path;
use tracing::{debug, event, info, span, Level};

const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_SUFFIX: &str = "Access";
const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_DOCUMENT: &str = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": [
                    "logs:CreateLogGroup",
                    "logs:CreateLogStream",
                    "logs:PutLogEvents",
                    "logs:DescribeLogStreams",
                    "s3:GetBucketLocation"
                ],
                "Resource": "*"
            }
        ]
    }"#;
const ROOT_CA_URL: &str = "https://www.amazontrust.com/repository/AmazonRootCA1.pem";
const IOT_ROLE_POLICY_NAME_PREFIX: &str = "GreengrassTESCertificatePolicy";
const GREENGRASS_CLI_COMPONENT_NAME: &str = "aws.greengrass.Cli";
const INITIAL_DEPLOYMENT_NAME_FORMAT: &str = "Deployment for %s";
const IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::%s:policy/%s";
const MANAGED_IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::aws:policy/%s";

const E2E_TESTS_POLICY_NAME_PREFIX: &str = "E2ETestsIotPolicy";
const E2E_TESTS_THING_NAME_PREFIX: &str = "E2ETestsIotThing";

// final Map<EnvironmentStage, String> tesServiceEndpoints = ImmutableMap.of(
//         EnvironmentStage.PROD, "credentials.iot.amazonaws.com",
//         EnvironmentStage.GAMMA, "credentials.iot.test.amazonaws.com",
//         EnvironmentStage.BETA, "credentials.iot.test.amazonaws.com"
// );

/*
 * Download root CA to a local file.
 *
 * To support HTTPS proxies and other custom truststore configurations, append to the file if it exists.
 */
pub async fn downloadRootCAToFile(path: &Path) {
    if Path::new(path).exists() {
        info!("Root CA file found at . Contents will be preserved.");
    }
    info!("Downloading Root CA from {}", ROOT_CA_URL);

    // TODO: append

    let body = reqwest::get(ROOT_CA_URL).await.unwrap().text().await;

    debug!("body = {:?}", &body);
    fs::write(path, body.unwrap()).expect("Unable to write file");

    // downloadFileFromURL(ROOT_CA_URL, path);
    // removeDuplicateCertificates(f);
    // Do not block as the root CA file may have been manually provisioned
    // info!("Failed to download Root CA.");
}

fn downloadFileFromURL(url: &str, path: &Path) {
    // let body = reqwest::get(url)
    // .await
    // .unwrap()
    // .text()
    // .await;

    // String certificates = new String(Files.readAllBytes(f.toPath()), StandardCharsets.UTF_8);
    // Set<String> uniqueCertificates =
    //         Arrays.stream(certificates.split(EncryptionUtils.CERTIFICATE_PEM_HEADER))
    //                 .map(s -> s.trim())
    //                 .collect(Collectors.toSet());

    // try (BufferedWriter bw = Files.newBufferedWriter(f.toPath(), StandardCharsets.UTF_8)) {
    //     for (String certificate : uniqueCertificates) {
    //         if (certificate.length() > 0) {
    //             bw.write(EncryptionUtils.CERTIFICATE_PEM_HEADER);
    //             bw.write("");
    //             bw.write(certificate);
    //             bw.write("");
    //         }
    //     }
    // }
    info!("Failed to remove duplicate certificates - %s%n");
}
// fn removeDuplicateCertificates() {
//     SdkHttpFullRequest request = SdkHttpFullRequest.builder()
//                 .uri(URI.create(url))
//                 .method(SdkHttpMethod.GET)
//                 .build();

//         HttpExecuteRequest executeRequest = HttpExecuteRequest.builder()
//                 .request(request)
//                 .build();

//         try (SdkHttpClient client = getSdkHttpClient()) {
//             HttpExecuteResponse executeResponse = client.prepareRequest(executeRequest).call();

//             int responseCode = executeResponse.httpResponse().statusCode();
//             if (responseCode != HttpURLConnection.HTTP_OK) {
//                 throw new IOException("Received invalid response code: " + responseCode);
//             }

//             try (InputStream inputStream = executeResponse.responseBody().get();
//                  OutputStream outputStream = Files.newOutputStream(f.toPath(), StandardOpenOption.CREATE,
//                          StandardOpenOption.APPEND)) {
//                 IoUtils.copy(inputStream, outputStream);
//             }
//         }}

pub async fn performSetup(needProvisioning: bool) {
    // // Describe usage of the command
    // if (showHelp) {
    //     info!(SHOW_HELP_RESPONSE);
    //     return;
    // }
    // if (showVersion) {
    //     // Use getVersionFromBuildMetadataFile so that we don't need to startup the Nucleus which is slow and will
    //     // start creating files and directories which may not be desired
    //     info!(String.format(SHOW_VERSION_RESPONSE,
    //             DeviceConfiguration.getVersionFromBuildRecipeFile()));
    //     return;
    // }

    // if (kernel == null) {
    //     kernel = new Kernel();
    // }
    // kernel.parseArgs(kernelArgs.toArray(new String[]{}));

    // try {
    //     IotSdkClientFactory.EnvironmentStage.fromString(environmentStage);
    // } catch (InvalidEnvironmentStageException e) {
    //     throw new RuntimeException(e);
    // }

    // if (!Utils.isEmpty(trustedPluginPaths)) {
    //     copyTrustedPlugins(kernel, trustedPluginPaths);
    // }
    // DeviceConfiguration deviceConfiguration = kernel.getContext().get(DeviceConfiguration.class);
    if (needProvisioning) {
        // if (Utils.isEmpty(awsRegion)) {
        //     awsRegion = Coerce.toString(deviceConfiguration.getAWSRegion());
        // }

        // if (Utils.isEmpty(awsRegion)) {
        //     throw new RuntimeException("Required input aws region not provided for provisioning");
        // }

        // this.deviceProvisioningHelper = new DeviceProvisioningHelper(awsRegion, environmentStage, this.outStream);
        // provision(kernel);
        provision();
    }

    // // Attempt this only after config file and Nucleus args have been parsed
    // setComponentDefaultUserAndGroup(deviceConfiguration);

    // if (setupSystemService) {
    //     kernel.getContext().get(KernelLifecycle.class).softShutdown(30);
    //     boolean ok = kernel.getContext().get(SystemServiceUtilsFactory.class).getInstance()
    //             .setupSystemService(kernel.getContext().get(KernelAlternatives.class));
    //     if (ok) {
    //         info!("Successfully set up Nucleus as a system service");
    //         // Nucleus will be launched by OS as a service
    //     } else {
    //         info!("Unable to set up Nucleus as a system service");
    //     }
    //     kernel.shutdown();
    //     return;
    // }
    // if (!kernelStart) {
    //     info!("Nucleus start set to false, exiting...");
    //     kernel.shutdown();
    //     return;
    // }
    info!("Launching Nucleus...");
    // kernel.launch();
    info!("Launched Nucleus successfully.");
}

async fn provision() {
    info!("Provisioning AWS IoT resources for the device with IoT Thing Name: [%s]...%n");
    // final ThingInfo thingInfo =
    //         deviceProvisioningHelper.createThing(deviceProvisioningHelper.getIotClient(), thingPolicyName,
    //                 thingName);
    info!("Successfully provisioned AWS IoT resources for the device with IoT Thing Name: [%s]!%n");
    // info!("Successfully provisioned AWS IoT resources for the device with IoT Thing Name: [%s]!%n",
    //         thingName);
    // if (!Utils.isEmpty(thingGroupName)) {
    //     info!("Adding IoT Thing [%s] into Thing Group: [%s]...%n", thingName, thingGroupName);
    //     deviceProvisioningHelper
    //             .addThingToGroup(deviceProvisioningHelper.getIotClient(), thingName, thingGroupName);
    //     info!("Successfully added Thing into Thing Group: [%s]%n", thingGroupName);
    // }
    // info!("Setting up resources for %s ... %n", TokenExchangeService.TOKEN_EXCHANGE_SERVICE_TOPICS);
    info!("Setting up resources for %s ... %n");
    // deviceProvisioningHelper.setupIoTRoleForTes(tesRoleName, tesRoleAliasName, thingInfo.getCertificateArn());
    // deviceProvisioningHelper.createAndAttachRolePolicy(tesRoleName, Region.of(awsRegion));
    info!("Configuring Nucleus with provisioned resource details...");
    // deviceProvisioningHelper.updateKernelConfigWithIotConfiguration(kernel, thingInfo, awsRegion, tesRoleAliasName);
    updateKernelConfigWithIotConfiguration();
    info!("Successfully configured Nucleus with provisioned resource details!");
    // if (deployDevTools) {
    //     deviceProvisioningHelper.createInitialDeploymentIfNeeded(thingInfo, thingGroupName,
    //             kernel.getContext().get(DeviceConfiguration.class).getNucleusVersion());
    // }

    // // Dump config since we've just provisioned so that the bootstrap config will enable us to
    // // reach the cloud when needed. Must do this now because we normally would never overwrite the bootstrap
    // // file, however we need to do it since we've only just learned about our endpoints, certs, etc.
    // kernel.writeEffectiveConfigAsTransactionLog(kernel.getNucleusPaths().configPath()
    //         .resolve(Kernel.DEFAULT_BOOTSTRAP_CONFIG_TLOG_FILE));
}

fn updateKernelConfigWithIotConfiguration() {
    // rootDir = kernel.getNucleusPaths().rootPath();
    let rootDir = Path::new("/greengrass/v2");
    let caFilePath = rootDir.join("rootCA.pem");
    let privKeyFilePath = rootDir.join("privKey.key");
    let certFilePath = rootDir.join("thingCert.crt");

    // downloadRootCAToFile(caFilePath.toFile());
    // try (CommitableFile cf = CommitableFile.of(privKeyFilePath, true)) {
    //     cf.write(thing.keyPair.privateKey().getBytes(StandardCharsets.UTF_8));
    // }
    // try (CommitableFile cf = CommitableFile.of(certFilePath, true)) {
    //     cf.write(thing.certificatePem.getBytes(StandardCharsets.UTF_8));
    // }

    // new DeviceConfiguration(kernel, thing.thingName, thing.dataEndpoint, thing.credEndpoint,
    //         privKeyFilePath.toString(), certFilePath.toString(), caFilePath.toString(), awsRegion, roleAliasName);
    // // Make sure tlog persists the device configuration
    // kernel.getContext().waitForPublishQueueToClear();
    // info!("Created device configuration");
}
pub async fn createThing(client: Client, policyName: &str, thingName: &str) {
    // provisioning::iot::create_thing();
    // Find or create IoT policy
    provisioning::iot::get_policy(&client, &policyName)
        .await
        .unwrap_or_else(|error| {
            info!("Error is {}", error);
        });
    info!("Found IoT policy \"%s\", reusing it%n");
    // info!("Creating new IoT policy \"%s\"%n", policyName);
    // client.createPolicy(CreatePolicyRequest.builder().policyName(policyName).policyDocument(
    //         "{\n  \"Version\": \"2012-10-17\",\n  \"Statement\": [\n    {\n"
    //                 + "      \"Effect\": \"Allow\",\n      \"Action\": [\n"
    //                 + "                \"iot:Connect\",\n                \"iot:Publish\",\n"
    //                 + "                \"iot:Subscribe\",\n                \"iot:Receive\",\n"
    //                 + "                \"greengrass:*\"\n],\n"
    //                 + "      \"Resource\": \"*\"\n    }\n  ]\n}")
    //         .build());

    // Create cert
    info!("Creating keys and certificate...");
    // CreateKeysAndCertificateResponse keyResponse =
    //         client.createKeysAndCertificate(CreateKeysAndCertificateRequest.builder().setAsActive(true).build());

    // Attach policy to cert
    info!("Attaching policy to certificate...");
    // client.attachPolicy(
    //         AttachPolicyRequest.builder().policyName(policyName).target(keyResponse.certificateArn()).build());

    // Create the thing and attach the cert to it
    info!("Creating IoT Thing \"%s\"...%n");
    // String thingArn = client.createThing(CreateThingRequest.builder().thingName(thingName).build()).thingArn();
    info!("Attaching certificate to IoT thing...");
    // client.attachThingPrincipal(
    //         AttachThingPrincipalRequest.builder().thingName(thingName).principal(keyResponse.certificateArn())
    //                 .build());

    // return new ThingInfo(thingArn, thingName, keyResponse.certificateArn(), keyResponse.certificateId(),
    //         keyResponse.certificatePem(), keyResponse.keyPair(),
    //         client.describeEndpoint(DescribeEndpointRequest.builder().endpointType("iot:Data-ATS").build())
    //                 .endpointAddress(), client.describeEndpoint(
    //         DescribeEndpointRequest.builder().endpointType("iot:CredentialProvider").build()).endpointAddress());
}