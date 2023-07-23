use serde::{Deserialize, Serialize};
use usiem::utilities::types::LogString;

use crate::err::TempResult;

pub async fn get_azure_ips() -> TempResult<AzureIpRanges> {
    let body = reqwest::get("https://download.microsoft.com/download/7/1/D/71D86715-5596-4529-9B13-DA13A5DE5B63/ServiceTags_Public_20230206.json")
    .await?
    .text()
    .await?;
    let res: AzureIpRanges = usiem::serde_json::from_str(&body)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzureIpRanges {
    #[serde(rename = "changeNumber")]
    pub change_number: u32,
    pub cloud: String,
    pub values: Vec<AzureService>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzureService {
    pub name: String,
    pub id: String,
    pub properties: AzureServiceProperties,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzureServiceProperties {
    #[serde(rename = "changeNumber")]
    pub change_number: u32,
    pub region: String,
    #[serde(rename = "regionId")]
    pub region_id: u32,
    pub platform: String,
    #[serde(rename = "systemService")]
    pub system_service: String,
    #[serde(rename = "addressPrefixes")]
    pub address_prefixes: Vec<String>,
}
pub fn static_region(region: &str) -> LogString {
    match region {
        "australiacentral" => LogString::Borrowed("Azure-australiacentral"),
        "australiacentral2" => LogString::Borrowed("Azure-australiacentral2"),
        "australiaeast" => LogString::Borrowed("Azure-australiaeast"),
        "australiasoutheast" => LogString::Borrowed("Azure-australiasoutheast"),
        "brazilsouth" => LogString::Borrowed("Azure-brazilsouth"),
        "brazilse" => LogString::Borrowed("Azure-brazilse"),
        "canadacentral" => LogString::Borrowed("Azure-canadacentral"),
        "canadaeast" => LogString::Borrowed("Azure-canadaeast"),
        "centralindia" => LogString::Borrowed("Azure-centralindia"),
        "centralus" => LogString::Borrowed("Azure-centralus"),
        "centraluseuap" => LogString::Borrowed("Azure-centraluseuap"),
        "eastasia" => LogString::Borrowed("Azure-eastasia"),
        "eastus" => LogString::Borrowed("Azure-eastus"),
        "eastus2" => LogString::Borrowed("Azure-eastus2"),
        "eastus2euap" => LogString::Borrowed("Azure-eastus2euap"),
        "centralfrance" => LogString::Borrowed("Azure-centralfrance"),
        "southfrance" => LogString::Borrowed("Azure-southfrance"),
        "germanyn" => LogString::Borrowed("Azure-germanyn"),
        "germanywc" => LogString::Borrowed("Azure-germanywc"),
        "japaneast" => LogString::Borrowed("Azure-japaneast"),
        "japanwest" => LogString::Borrowed("Azure-japanwest"),
        "jioindiacentral" => LogString::Borrowed("Azure-jioindiacentral"),
        "jioindiawest" => LogString::Borrowed("Azure-jioindiawest"),
        "koreacentral" => LogString::Borrowed("Azure-koreacentral"),
        "northcentralus" => LogString::Borrowed("Azure-northcentralus"),
        "northeurope" => LogString::Borrowed("Azure-northeurope"),
        "norwaye" => LogString::Borrowed("Azure-norwaye"),
        "norwayw" => LogString::Borrowed("Azure-norwayw"),
        "qatarcentral" => LogString::Borrowed("Azure-qatarcentral"),
        "southafricanorth" => LogString::Borrowed("Azure-southafricanorth"),
        "southafricawest" => LogString::Borrowed("Azure-southafricawest"),
        "southcentralus" => LogString::Borrowed("Azure-southcentralus"),
        "southindia" => LogString::Borrowed("Azure-southindia"),
        "southeastasia" => LogString::Borrowed("Azure-southeastasia"),
        "swedencentral" => LogString::Borrowed("Azure-swedencentral"),
        "swedensouth" => LogString::Borrowed("Azure-swedensouth"),
        "switzerlandn" => LogString::Borrowed("Azure-switzerlandn"),
        "switzerlandw" => LogString::Borrowed("Azure-switzerlandw"),
        "uaecentral" => LogString::Borrowed("Azure-uaecentral"),
        "uaenorth" => LogString::Borrowed("Azure-uaenorth"),
        "uknorth" => LogString::Borrowed("Azure-uknorth"),
        "uksouth" => LogString::Borrowed("Azure-uksouth"),
        "uksouth2" => LogString::Borrowed("Azure-uksouth2"),
        "ukwest" => LogString::Borrowed("Azure-ukwest"),
        "westcentralus" => LogString::Borrowed("Azure-westcentralus"),
        "westeurope" => LogString::Borrowed("Azure-westeurope"),
        "westindia" => LogString::Borrowed("Azure-westindia"),
        "westus" => LogString::Borrowed("Azure-westus"),
        "westus2" => LogString::Borrowed("Azure-westus2"),
        "westus3" => LogString::Borrowed("Azure-westus3"),
        "koreasouth" => LogString::Borrowed("Azure-koreasouth"),
        "usstagec" => LogString::Borrowed("Azure-usstagec"),
        "brazilne" => LogString::Borrowed("Azure-brazilne"),
        "northeurope2" => LogString::Borrowed("Azure-northeurope2"),
        _ => LogString::Owned(region.to_string()),
    }
}

pub fn static_service(service: &str) -> LogString {
    match service {
        "ActionGroup" => LogString::Borrowed("AzureActionGroup"),
        "ApplicationInsightsAvailability" => {
            LogString::Borrowed("AzureApplicationInsightsAvailability")
        }
        "AutonomousDevelopmentPlatform" => {
            LogString::Borrowed("AzureAutonomousDevelopmentPlatform")
        }
        "AzureAD" => LogString::Borrowed("AzureAD"),
        "AzureAPIForFHIR" => LogString::Borrowed("AzureAPIForFHIR"),
        "AzureAdvancedThreatProtection" => LogString::Borrowed("AzureAdvancedThreatProtection"),
        "AzureApiManagement" => LogString::Borrowed("AzureApiManagement"),
        "AzureAppConfiguration" => LogString::Borrowed("AzureAppConfiguration"),
        "AzureAppService" => LogString::Borrowed("AzureAppService"),
        "AzureAppServiceManagement" => LogString::Borrowed("AzureAppServiceManagement"),
        "AzureArcInfrastructure" => LogString::Borrowed("AzureArcInfrastructure"),
        "AzureAttestation" => LogString::Borrowed("AzureAttestation"),
        "AzureAutomation" => LogString::Borrowed("AzureAutomation"),
        "AzureBackup" => LogString::Borrowed("AzureBackup"),
        "AzureBotService" => LogString::Borrowed("AzureBotService"),
        "AzureCognitiveSearch" => LogString::Borrowed("AzureCognitiveSearch"),
        "AzureConnectors" => LogString::Borrowed("AzureConnectors"),
        "AzureContainerRegistry" => LogString::Borrowed("AzureContainerRegistry"),
        "AzureCosmosDB" => LogString::Borrowed("AzureCosmosDB"),
        "AzureDataExplorerManagement" => LogString::Borrowed("AzureDataExplorerManagement"),
        "AzureDataLake" => LogString::Borrowed("AzureDataLake"),
        "AzureDatabricks" => LogString::Borrowed("AzureDatabricks"),
        "AzureDevOps" => LogString::Borrowed("AzureDevOps"),
        "AzureDevSpaces" => LogString::Borrowed("AzureDevSpaces"),
        "AzureDigitalTwins" => LogString::Borrowed("AzureDigitalTwins"),
        "AzureEventGrid" => LogString::Borrowed("AzureEventGrid"),
        "AzureEventHub" => LogString::Borrowed("AzureEventHub"),
        "AzureInformationProtection" => LogString::Borrowed("AzureInformationProtection"),
        "AzureIoTHub" => LogString::Borrowed("AzureIoTHub"),
        "AzureKeyVault" => LogString::Borrowed("AzureKeyVault"),
        "AzureLoadTestingInstanceManagement" => {
            LogString::Borrowed("AzureLoadTestingInstanceManagement")
        }
        "AzureMachineLearning" => LogString::Borrowed("AzureMachineLearning"),
        "AzureMonitor" => LogString::Borrowed("AzureMonitor"),
        "AzureOpenDatasets" => LogString::Borrowed("AzureOpenDatasets"),
        "AzurePortal" => LogString::Borrowed("AzurePortal"),
        "AzureResourceManager" => LogString::Borrowed("AzureResourceManager"),
        "AzureSQL" => LogString::Borrowed("AzureSQL"),
        "AzureSecurityCenter" => LogString::Borrowed("AzureSecurityCenter"),
        "AzureSentinel" => LogString::Borrowed("AzureSentinel"),
        "AzureServiceBus" => LogString::Borrowed("AzureServiceBus"),
        "AzureSignalR" => LogString::Borrowed("AzureSignalR"),
        "AzureSiteRecovery" => LogString::Borrowed("AzureSiteRecovery"),
        "AzureSphereSecureService_Prod" => LogString::Borrowed("AzureSphereSecureService_Prod"),
        "AzureStack" => LogString::Borrowed("AzureStack"),
        "AzureStorage" => LogString::Borrowed("AzureStorage"),
        "AzureTrafficManager" => LogString::Borrowed("AzureTrafficManager"),
        "AzureUpdateDelivery" => LogString::Borrowed("AzureUpdateDelivery"),
        "AzureWebPubSub" => LogString::Borrowed("AzureWebPubSub"),
        "BatchNodeManagement" => LogString::Borrowed("AzureBatchNodeManagement"),
        "ChaosStudio" => LogString::Borrowed("AzureChaosStudio"),
        "CognitiveServicesManagement" => LogString::Borrowed("AzureCognitiveServicesManagement"),
        "DataFactory" => LogString::Borrowed("AzureDataFactory"),
        "Dynamics365BusinessCentral" => LogString::Borrowed("AzureDynamics365BusinessCentral"),
        "Dynamics365ForMarketingEmail" => LogString::Borrowed("AzureDynamics365ForMarketingEmail"),
        "EOPExtPublished" => LogString::Borrowed("AzureEOPExtPublished"),
        "GatewayManager" => LogString::Borrowed("AzureGatewayManager"),
        "Grafana" => LogString::Borrowed("AzureGrafana"),
        "HDInsight" => LogString::Borrowed("AzureHDInsight"),
        "LogicApps" => LogString::Borrowed("AzureLogicApps"),
        "M365ManagementActivityApi" => LogString::Borrowed("AzureM365ManagementActivityApi"),
        "M365ManagementActivityApiWebhook" => {
            LogString::Borrowed("AzureM365ManagementActivityApiWebhook")
        }
        "MicrosoftAzureFluidRelay" => LogString::Borrowed("AzureMicrosoftAzureFluidRelay"),
        "MicrosoftCloudAppSecurity" => LogString::Borrowed("AzureMicrosoftCloudAppSecurity"),
        "MicrosoftContainerRegistry" => LogString::Borrowed("AzureMicrosoftContainerRegistry"),
        "MicrosoftDefenderForEndpoint" => LogString::Borrowed("AzureMicrosoftDefenderForEndpoint"),
        "OneDsCollector" => LogString::Borrowed("AzureOneDsCollector"),
        "PowerBI" => LogString::Borrowed("AzurePowerBI"),
        "PowerPlatformInfra" => LogString::Borrowed("AzurePowerPlatformInfra"),
        "PowerPlatformPlex" => LogString::Borrowed("AzurePowerPlatformPlex"),
        "PowerQueryOnline" => LogString::Borrowed("AzurePowerQueryOnline"),
        "SCCservice" => LogString::Borrowed("AzureSCCservice"),
        "ServiceFabric" => LogString::Borrowed("AzureServiceFabric"),
        "SqlManagement" => LogString::Borrowed("AzureSqlManagement"),
        "StorageSyncService" => LogString::Borrowed("AzureStorageSyncService"),
        "WindowsAdminCenter" => LogString::Borrowed("AzureWindowsAdminCenter"),
        "WindowsVirtualDesktop" => LogString::Borrowed("AzureWindowsVirtualDesktop"),
        "AzureFrontDoor" => LogString::Borrowed("AzureFrontDoor"),
        "AzureIdentity" => LogString::Borrowed("AzureIdentity"),
        _ => LogString::Owned(service.to_string()),
    }
}

#[tokio::test]
async fn test_azure() {
    let _res = get_azure_ips().await;
}
