#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceNameAvailability {
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<resource_name_availability::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod resource_name_availability {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: resource_name_availability_request::Type,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
pub mod resource_name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "activeDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub active_directories: Vec<ActiveDirectory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectory {
    #[serde(rename = "activeDirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "smbServerName", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_name: Option<String>,
    #[serde(rename = "organizationalUnit", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizeRequest {
    #[serde(rename = "remoteVolumeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPool {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: PoolProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapacityPool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityPoolPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportPolicyRule {
    #[serde(rename = "ruleIndex", default, skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<i64>,
    #[serde(rename = "unixReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_only: Option<bool>,
    #[serde(rename = "unixReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_write: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cifs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv3: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv41: Option<bool>,
    #[serde(rename = "allowedClients", default, skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTarget {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: MountTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTargetProperties {
    #[serde(rename = "mountTargetId", default, skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[serde(rename = "startIp", default, skip_serializing_if = "Option::is_none")]
    pub start_ip: Option<String>,
    #[serde(rename = "endIp", default, skip_serializing_if = "Option::is_none")]
    pub end_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(rename = "smbServerFqdn", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_fqdn: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccount {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetAppAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetAppAccountPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<pool_patch_properties::ServiceLevel>,
}
pub mod pool_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    pub size: i64,
    #[serde(rename = "serviceLevel")]
    pub service_level: pool_properties::ServiceLevel,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
pub mod pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationObject {
    #[serde(rename = "replicationId", default, skip_serializing_if = "Option::is_none")]
    pub replication_id: Option<String>,
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<replication_object::EndpointType>,
    #[serde(rename = "replicationSchedule")]
    pub replication_schedule: replication_object::ReplicationSchedule,
    #[serde(rename = "remoteVolumeResourceId")]
    pub remote_volume_resource_id: String,
    #[serde(rename = "remoteVolumeRegion", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_region: Option<String>,
}
pub mod replication_object {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        #[serde(rename = "src")]
        Src,
        #[serde(rename = "dst")]
        Dst,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReplicationSchedule {
        #[serde(rename = "_10minutely")]
        N10minutely,
        #[serde(rename = "hourly")]
        Hourly,
        #[serde(rename = "daily")]
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<replication_status::RelationshipStatus>,
    #[serde(rename = "mirrorState", default, skip_serializing_if = "Option::is_none")]
    pub mirror_state: Option<replication_status::MirrorState>,
    #[serde(rename = "totalProgress", default, skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
pub mod replication_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RelationshipStatus {
        Idle,
        Transferring,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MirrorState {
        Uninitialized,
        Mirrored,
        Broken,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceTags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotPatch {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotProperties {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Snapshot>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: VolumeProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Volume>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumePatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatchProperties {
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_patch_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold", default, skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<i64>,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_patch_properties::ExportPolicy>,
}
pub mod volume_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "creationToken")]
    pub creation_token: String,
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold")]
    pub usage_threshold: i64,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_properties::ExportPolicy>,
    #[serde(rename = "protocolTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_types: Vec<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "baremetalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub baremetal_tenant_id: Option<String>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "mountTargets", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_targets: Vec<MountTargetProperties>,
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_properties::DataProtection>,
    #[serde(rename = "isRestoring", default, skip_serializing_if = "Option::is_none")]
    pub is_restoring: Option<bool>,
}
pub mod volume_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    impl Default for ServiceLevel {
        fn default() -> Self {
            Self::Premium
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataProtection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replication: Option<ReplicationObject>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeRevert {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}
