#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<api_operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_operation::Properties>,
}
pub mod api_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<properties::ServiceSpecification>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ServiceSpecification {
            #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
            pub metric_specifications: Vec<MetricSpecification>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperationListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscOperation {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AscOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AscOperationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cache {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UrlString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<CacheIdentity>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cache::Properties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<cache::Sku>,
}
pub mod cache {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "cacheSizeGB", default, skip_serializing_if = "Option::is_none")]
        pub cache_size_gb: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub health: Option<CacheHealth>,
        #[serde(rename = "mountAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub mount_addresses: Vec<String>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet: Option<UrlString>,
        #[serde(rename = "upgradeStatus", default, skip_serializing_if = "Option::is_none")]
        pub upgrade_status: Option<CacheUpgradeStatus>,
        #[serde(rename = "networkSettings", default, skip_serializing_if = "Option::is_none")]
        pub network_settings: Option<CacheNetworkSettings>,
        #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
        pub encryption_settings: Option<CacheEncryptionSettings>,
        #[serde(rename = "securitySettings", default, skip_serializing_if = "Option::is_none")]
        pub security_settings: Option<CacheSecuritySettings>,
        #[serde(rename = "directoryServicesSettings", default, skip_serializing_if = "Option::is_none")]
        pub directory_services_settings: Option<CacheDirectorySettings>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Cancelled,
            Creating,
            Deleting,
            Updating,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Sku {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheActiveDirectorySettings {
    #[serde(rename = "primaryDnsIpAddress")]
    pub primary_dns_ip_address: String,
    #[serde(rename = "secondaryDnsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub secondary_dns_ip_address: Option<String>,
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[serde(rename = "domainNetBiosName")]
    pub domain_net_bios_name: String,
    #[serde(rename = "cacheNetBiosName")]
    pub cache_net_bios_name: String,
    #[serde(rename = "domainJoined", default, skip_serializing_if = "Option::is_none")]
    pub domain_joined: Option<cache_active_directory_settings::DomainJoined>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_active_directory_settings::Credentials>,
}
pub mod cache_active_directory_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DomainJoined {
        Yes,
        No,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheDirectorySettings {
    #[serde(rename = "activeDirectory", default, skip_serializing_if = "Option::is_none")]
    pub active_directory: Option<CacheActiveDirectorySettings>,
    #[serde(rename = "usernameDownload", default, skip_serializing_if = "Option::is_none")]
    pub username_download: Option<CacheUsernameDownloadSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheEncryptionSettings {
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheHealth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cache_health::State>,
    #[serde(rename = "statusDescription", default, skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}
pub mod cache_health {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Unknown,
        Healthy,
        Degraded,
        Down,
        Transitioning,
        Stopping,
        Stopped,
        Upgrading,
        Flushing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<cache_identity::Type>,
}
pub mod cache_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheNetworkSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "utilityAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub utility_addresses: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheSecuritySettings {
    #[serde(rename = "accessPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<NfsAccessPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUpgradeStatus {
    #[serde(rename = "currentFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_firmware_version: Option<String>,
    #[serde(rename = "firmwareUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub firmware_update_status: Option<cache_upgrade_status::FirmwareUpdateStatus>,
    #[serde(rename = "firmwareUpdateDeadline", default, skip_serializing_if = "Option::is_none")]
    pub firmware_update_deadline: Option<String>,
    #[serde(rename = "lastFirmwareUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_firmware_update: Option<String>,
    #[serde(rename = "pendingFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub pending_firmware_version: Option<String>,
}
pub mod cache_upgrade_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirmwareUpdateStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUsernameDownloadSettings {
    #[serde(rename = "extendedGroups", default, skip_serializing_if = "Option::is_none")]
    pub extended_groups: Option<bool>,
    #[serde(rename = "usernameSource", default, skip_serializing_if = "Option::is_none")]
    pub username_source: Option<cache_username_download_settings::UsernameSource>,
    #[serde(rename = "groupFileURI", default, skip_serializing_if = "Option::is_none")]
    pub group_file_uri: Option<String>,
    #[serde(rename = "userFileURI", default, skip_serializing_if = "Option::is_none")]
    pub user_file_uri: Option<String>,
    #[serde(rename = "ldapServer", default, skip_serializing_if = "Option::is_none")]
    pub ldap_server: Option<String>,
    #[serde(rename = "ldapBaseDN", default, skip_serializing_if = "Option::is_none")]
    pub ldap_base_dn: Option<String>,
    #[serde(rename = "encryptLdapConnection", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_ldap_connection: Option<bool>,
    #[serde(rename = "requireValidCertificate", default, skip_serializing_if = "Option::is_none")]
    pub require_valid_certificate: Option<bool>,
    #[serde(rename = "autoDownloadCertificate", default, skip_serializing_if = "Option::is_none")]
    pub auto_download_certificate: Option<bool>,
    #[serde(rename = "caCertificateURI", default, skip_serializing_if = "Option::is_none")]
    pub ca_certificate_uri: Option<String>,
    #[serde(rename = "usernameDownloaded", default, skip_serializing_if = "Option::is_none")]
    pub username_downloaded: Option<cache_username_download_settings::UsernameDownloaded>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cache_username_download_settings::Credentials>,
}
pub mod cache_username_download_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UsernameSource {
        #[serde(rename = "AD")]
        Ad,
        #[serde(rename = "LDAP")]
        Ldap,
        File,
        None,
    }
    impl Default for UsernameSource {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UsernameDownloaded {
        Yes,
        No,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Credentials {
        #[serde(rename = "bindDn", default, skip_serializing_if = "Option::is_none")]
        pub bind_dn: Option<String>,
        #[serde(rename = "bindPassword", default, skip_serializing_if = "Option::is_none")]
        pub bind_password: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CachesListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cache>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClfsTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<UrlString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClfsTargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[serde(rename = "keyUrl")]
    pub key_url: String,
    #[serde(rename = "sourceVault")]
    pub source_vault: key_vault_key_reference::SourceVault,
}
pub mod key_vault_key_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SourceVault {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[serde(rename = "metricClass", default, skip_serializing_if = "Option::is_none")]
    pub metric_class: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceJunction {
    #[serde(rename = "namespacePath", default, skip_serializing_if = "Option::is_none")]
    pub namespace_path: Option<String>,
    #[serde(rename = "targetPath", default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(rename = "nfsExport", default, skip_serializing_if = "Option::is_none")]
    pub nfs_export: Option<String>,
    #[serde(rename = "nfsAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub nfs_access_policy: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Nfs3Target {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "usageModel", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nfs3TargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessPolicy {
    pub name: String,
    #[serde(rename = "accessRules")]
    pub access_rules: Vec<NfsAccessRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfsAccessRule {
    pub scope: nfs_access_rule::Scope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    pub access: nfs_access_rule::Access,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suid: Option<bool>,
    #[serde(rename = "submountAccess", default, skip_serializing_if = "Option::is_none")]
    pub submount_access: Option<bool>,
    #[serde(rename = "rootSquash", default, skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<bool>,
    #[serde(rename = "anonymousUID", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_uid: Option<String>,
    #[serde(rename = "anonymousGID", default, skip_serializing_if = "Option::is_none")]
    pub anonymous_gid: Option<String>,
}
pub mod nfs_access_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "network")]
        Network,
        #[serde(rename = "host")]
        Host,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        #[serde(rename = "no")]
        No,
        #[serde(rename = "ro")]
        Ro,
        #[serde(rename = "rw")]
        Rw,
    }
}
pub type ResourceName = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkusResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Restriction {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
pub mod restriction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTarget {
    #[serde(flatten)]
    pub storage_target_resource: StorageTargetResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTargetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub junctions: Vec<NamespaceJunction>,
    #[serde(rename = "targetType")]
    pub target_type: storage_target_properties::TargetType,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_target_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs3: Option<Nfs3Target>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clfs: Option<ClfsTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<UnknownTarget>,
}
pub mod storage_target_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetType {
        #[serde(rename = "nfs3")]
        Nfs3,
        #[serde(rename = "clfs")]
        Clfs,
        #[serde(rename = "unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Cancelled,
        Creating,
        Deleting,
        Updating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTargetResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTargetsResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageTarget>,
}
pub type UrlString = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnknownProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnknownTarget {
    #[serde(rename = "unknownMap", default, skip_serializing_if = "Option::is_none")]
    pub unknown_map: Option<UnknownProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownTargetProperties {
    #[serde(flatten)]
    pub storage_target_properties: StorageTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<usage_model::Display>,
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}
pub mod usage_model {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageModelsResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageModel>,
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
