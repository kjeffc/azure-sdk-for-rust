#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerMutableProperties {
    #[serde(rename = "asAdministrators", default, skip_serializing_if = "Option::is_none")]
    pub as_administrators: Option<ServerAdministrators>,
    #[serde(rename = "backupBlobContainerUri", default, skip_serializing_if = "Option::is_none")]
    pub backup_blob_container_uri: Option<String>,
    #[serde(rename = "gatewayDetails", default, skip_serializing_if = "Option::is_none")]
    pub gateway_details: Option<GatewayDetails>,
    #[serde(rename = "ipV4FirewallSettings", default, skip_serializing_if = "Option::is_none")]
    pub ip_v4_firewall_settings: Option<IPv4FirewallSettings>,
    #[serde(rename = "querypoolConnectionMode", default, skip_serializing_if = "Option::is_none")]
    pub querypool_connection_mode: Option<analysis_services_server_mutable_properties::QuerypoolConnectionMode>,
    #[serde(rename = "managedMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_mode: Option<analysis_services_server_mutable_properties::ManagedMode>,
    #[serde(rename = "serverMonitorMode", default, skip_serializing_if = "Option::is_none")]
    pub server_monitor_mode: Option<analysis_services_server_mutable_properties::ServerMonitorMode>,
}
pub mod analysis_services_server_mutable_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QuerypoolConnectionMode {
        All,
        ReadOnly,
    }
    impl Default for QuerypoolConnectionMode {
        fn default() -> Self {
            Self::All
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManagedMode {}
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerMonitorMode {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerProperties {
    #[serde(flatten)]
    pub analysis_services_server_mutable_properties: AnalysisServicesServerMutableProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<analysis_services_server_properties::State>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<analysis_services_server_properties::ProvisioningState>,
    #[serde(rename = "serverFullName", default, skip_serializing_if = "Option::is_none")]
    pub server_full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
pub mod analysis_services_server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalysisServicesServerUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServers {
    pub value: Vec<AnalysisServicesServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServerNameAvailabilityParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckServerNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetail>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayDetails {
    #[serde(rename = "gatewayResourceId", default, skip_serializing_if = "Option::is_none")]
    pub gateway_resource_id: Option<String>,
    #[serde(rename = "gatewayObjectId", default, skip_serializing_if = "Option::is_none")]
    pub gateway_object_id: Option<String>,
    #[serde(rename = "dmtsClusterUri", default, skip_serializing_if = "Option::is_none")]
    pub dmts_cluster_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayListStatusError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<GatewayError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayListStatusLive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<gateway_list_status_live::Status>,
}
pub mod gateway_list_status_live {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv4FirewallRule {
    #[serde(rename = "firewallRuleName", default, skip_serializing_if = "Option::is_none")]
    pub firewall_rule_name: Option<String>,
    #[serde(rename = "rangeStart", default, skip_serializing_if = "Option::is_none")]
    pub range_start: Option<String>,
    #[serde(rename = "rangeEnd", default, skip_serializing_if = "Option::is_none")]
    pub range_end: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IPv4FirewallSettings {
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<IPv4FirewallRule>,
    #[serde(rename = "enablePowerBIService", default, skip_serializing_if = "Option::is_none")]
    pub enable_power_bi_service: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    pub sku: ResourceSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<resource_sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod resource_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Development,
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAdministrators {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDetailsForExistingResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForExistingResourceResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDetailsForExistingResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuEnumerationForNewResourceResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
