#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Backup {
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<backup::GeoRedundantBackup>,
    #[serde(rename = "earliestRestoreDate", default, skip_serializing_if = "Option::is_none")]
    pub earliest_restore_date: Option<String>,
}
pub mod backup {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GeoRedundantBackup {
        Enabled,
        Disabled,
    }
    impl Default for GeoRedundantBackup {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilitiesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapabilityProperties>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(rename = "geoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub geo_backup_supported: Option<bool>,
    #[serde(rename = "zoneRedundantHaSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_supported: Option<bool>,
    #[serde(rename = "zoneRedundantHaAndGeoBackupSupported", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant_ha_and_geo_backup_supported: Option<bool>,
    #[serde(rename = "supportedFlexibleServerEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_flexible_server_editions: Vec<FlexibleServerEditionCapability>,
    #[serde(rename = "supportedHyperscaleNodeEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_hyperscale_node_editions: Vec<HyperscaleNodeEditionCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<configuration_properties::DataType>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "isDynamicConfig", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_config: Option<bool>,
    #[serde(rename = "isReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "isConfigPendingRestart", default, skip_serializing_if = "Option::is_none")]
    pub is_config_pending_restart: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "documentationLink", default, skip_serializing_if = "Option::is_none")]
    pub documentation_link: Option<String>,
}
pub mod configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataType {
        Boolean,
        Numeric,
        Integer,
        Enumeration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelegatedSubnetUsage {
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: FirewallRuleProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlexibleServerEditionCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportedStorageEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[serde(rename = "supportedServerVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HighAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<high_availability::Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<high_availability::State>,
    #[serde(rename = "standbyAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub standby_availability_zone: Option<String>,
}
pub mod high_availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Disabled,
        ZoneRedundant,
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        NotEnabled,
        CreatingStandby,
        ReplicatingData,
        FailingOver,
        Healthy,
        RemovingStandby,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperscaleNodeEditionCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportedStorageEditions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_storage_editions: Vec<StorageEditionCapability>,
    #[serde(rename = "supportedServerVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_server_versions: Vec<ServerVersionCapability>,
    #[serde(rename = "supportedNodeTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_node_types: Vec<NodeTypeCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceWindow {
    #[serde(rename = "customWindow", default, skip_serializing_if = "Option::is_none")]
    pub custom_window: Option<String>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i32>,
    #[serde(rename = "startMinute", default, skip_serializing_if = "Option::is_none")]
    pub start_minute: Option<i32>,
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<name_availability::Reason>,
}
pub mod name_availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<network::PublicNetworkAccess>,
    #[serde(rename = "delegatedSubnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_subnet_resource_id: Option<String>,
    #[serde(rename = "privateDnsZoneArmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_dns_zone_arm_resource_id: Option<String>,
}
pub mod network {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeTypeCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeType", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
pub type PrivateDnsZoneSuffix = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestartParameter {
    #[serde(rename = "restartWithFailover", default, skip_serializing_if = "Option::is_none")]
    pub restart_with_failover: Option<bool>,
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<restart_parameter::FailoverMode>,
}
pub mod restart_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FailoverMode {
        PlannedFailover,
        ForcedFailover,
        PlannedSwitchover,
        ForcedSwitchover,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerForUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerPropertiesForUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "minorVersion", default, skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<server_properties::State>,
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[serde(rename = "sourceServerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_server_resource_id: Option<String>,
    #[serde(rename = "pointInTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub point_in_time_utc: Option<String>,
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties::CreateMode>,
}
pub mod server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Ready,
        Dropping,
        Disabled,
        Starting,
        Stopping,
        Stopped,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        Create,
        Update,
        PointInTimeRestore,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPropertiesForUpdate {
    #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<MaintenanceWindow>,
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<server_properties_for_update::CreateMode>,
}
pub mod server_properties_for_update {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        Update,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerVersion {
    #[serde(rename = "13")]
    N13,
    #[serde(rename = "12")]
    N12,
    #[serde(rename = "11")]
    N11,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerVersionCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportedVcores", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_vcores: Vec<VcoreCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    pub tier: sku::Tier,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Burstable,
        GeneralPurpose,
        MemoryOptimized,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Storage {
    #[serde(rename = "storageSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_gb: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageEditionCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportedStorageMB", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_storage_mb: Vec<StorageMbCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMbCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i64>,
    #[serde(rename = "storageSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_size_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VcoreCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "vCores", default, skip_serializing_if = "Option::is_none")]
    pub v_cores: Option<i64>,
    #[serde(rename = "supportedIops", default, skip_serializing_if = "Option::is_none")]
    pub supported_iops: Option<i64>,
    #[serde(rename = "supportedMemoryPerVcoreMB", default, skip_serializing_if = "Option::is_none")]
    pub supported_memory_per_vcore_mb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageParameter {
    #[serde(rename = "virtualNetworkArmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_arm_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkSubnetUsageResult {
    #[serde(rename = "delegatedSubnetsUsage", default, skip_serializing_if = "Vec::is_empty")]
    pub delegated_subnets_usage: Vec<DelegatedSubnetUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
