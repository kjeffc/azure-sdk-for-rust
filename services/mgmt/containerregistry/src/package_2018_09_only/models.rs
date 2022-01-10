#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub value: String,
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "tokenType")]
    pub token_type: auth_info::TokenType,
    pub token: String,
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}
pub mod auth_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TokenType {
        #[serde(rename = "PAT")]
        Pat,
        OAuth,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthInfoUpdateParameters {
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<auth_info_update_parameters::TokenType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "expiresIn", default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}
pub mod auth_info_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TokenType {
        #[serde(rename = "PAT")]
        Pat,
        OAuth,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseImageDependency {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<base_image_dependency::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
pub mod base_image_dependency {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        BuildTime,
        RunTime,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseImageTrigger {
    #[serde(rename = "baseImageTriggerType")]
    pub base_image_trigger_type: base_image_trigger::BaseImageTriggerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<base_image_trigger::Status>,
    pub name: String,
}
pub mod base_image_trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BaseImageTriggerType {
        All,
        Runtime,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseImageTriggerUpdateParameters {
    #[serde(rename = "baseImageTriggerType", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger_type: Option<base_image_trigger_update_parameters::BaseImageTriggerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<base_image_trigger_update_parameters::Status>,
    pub name: String,
}
pub mod base_image_trigger_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BaseImageTriggerType {
        All,
        Runtime,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Credentials {
    #[serde(rename = "sourceRegistry", default, skip_serializing_if = "Option::is_none")]
    pub source_registry: Option<SourceRegistryCredentials>,
    #[serde(rename = "customRegistries", default, skip_serializing_if = "Option::is_none")]
    pub custom_registries: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRegistryCredentials {
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<SecretObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<SecretObject>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[serde(rename = "dockerFilePath")]
    pub docker_file_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    pub platform: PlatformProperties,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[serde(rename = "dockerFilePath")]
    pub docker_file_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DockerBuildStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[serde(rename = "imageNames", default, skip_serializing_if = "Vec::is_empty")]
    pub image_names: Vec<String>,
    #[serde(rename = "isPushEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_push_enabled: Option<bool>,
    #[serde(rename = "noCache", default, skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
    #[serde(rename = "dockerFilePath", default, skip_serializing_if = "Option::is_none")]
    pub docker_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Argument>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedTaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[serde(rename = "encodedTaskContent")]
    pub encoded_task_content: String,
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    pub platform: PlatformProperties,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedTaskStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[serde(rename = "encodedTaskContent")]
    pub encoded_task_content: String,
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncodedTaskStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[serde(rename = "encodedTaskContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_task_content: Option<String>,
    #[serde(rename = "encodedValuesContent", default, skip_serializing_if = "Option::is_none")]
    pub encoded_values_content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[serde(rename = "taskFilePath")]
    pub task_file_path: String,
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    pub platform: PlatformProperties,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(rename = "sourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTaskStep {
    #[serde(flatten)]
    pub task_step_properties: TaskStepProperties,
    #[serde(rename = "taskFilePath")]
    pub task_file_path: String,
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileTaskStepUpdateParameters {
    #[serde(flatten)]
    pub task_step_update_parameters: TaskStepUpdateParameters,
    #[serde(rename = "taskFilePath", default, skip_serializing_if = "Option::is_none")]
    pub task_file_path: Option<String>,
    #[serde(rename = "valuesFilePath", default, skip_serializing_if = "Option::is_none")]
    pub values_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdateTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ImageDescriptor>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformProperties {
    pub os: platform_properties::Os,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<platform_properties::Architecture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<platform_properties::Variant>,
}
pub mod platform_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Os {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Architecture {
        #[serde(rename = "amd64")]
        Amd64,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Variant {
        #[serde(rename = "v6")]
        V6,
        #[serde(rename = "v7")]
        V7,
        #[serde(rename = "v8")]
        V8,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlatformUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<platform_update_parameters::Os>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<platform_update_parameters::Architecture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<platform_update_parameters::Variant>,
}
pub mod platform_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Os {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Architecture {
        #[serde(rename = "amd64")]
        Amd64,
        #[serde(rename = "x86")]
        X86,
        #[serde(rename = "arm")]
        Arm,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Variant {
        #[serde(rename = "v6")]
        V6,
        #[serde(rename = "v7")]
        V7,
        #[serde(rename = "v8")]
        V8,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Run {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunFilter {
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<run_filter::RunType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<run_filter::Status>,
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "outputImageManifests", default, skip_serializing_if = "Option::is_none")]
    pub output_image_manifests: Option<String>,
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
}
pub mod run_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunType {
        QuickBuild,
        QuickRun,
        AutoBuild,
        AutoRun,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Queued,
        Started,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Error,
        Timeout,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunGetLogResult {
    #[serde(rename = "logLink", default, skip_serializing_if = "Option::is_none")]
    pub log_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Run>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunProperties {
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<run_properties::Status>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "runType", default, skip_serializing_if = "Option::is_none")]
    pub run_type: Option<run_properties::RunType>,
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "outputImages", default, skip_serializing_if = "Vec::is_empty")]
    pub output_images: Vec<ImageDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(rename = "imageUpdateTrigger", default, skip_serializing_if = "Option::is_none")]
    pub image_update_trigger: Option<ImageUpdateTrigger>,
    #[serde(rename = "sourceTrigger", default, skip_serializing_if = "Option::is_none")]
    pub source_trigger: Option<SourceTriggerDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformProperties>,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(rename = "sourceRegistryAuth", default, skip_serializing_if = "Option::is_none")]
    pub source_registry_auth: Option<String>,
    #[serde(rename = "customRegistries", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_registries: Vec<String>,
    #[serde(rename = "runErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub run_error_message: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<run_properties::ProvisioningState>,
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
}
pub mod run_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Queued,
        Started,
        Running,
        Succeeded,
        Failed,
        Canceled,
        Error,
        Timeout,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunType {
        QuickBuild,
        QuickRun,
        AutoBuild,
        AutoRun,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunRequest {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunUpdateParameters {
    #[serde(rename = "isArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_archive_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<secret_object::Type>,
}
pub mod secret_object {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Opaque,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetValue {
    pub name: String,
    pub value: String,
    #[serde(rename = "isSecret", default, skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceProperties {
    #[serde(rename = "sourceControlType")]
    pub source_control_type: source_properties::SourceControlType,
    #[serde(rename = "repositoryUrl")]
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "sourceControlAuthProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_control_auth_properties: Option<AuthInfo>,
}
pub mod source_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceControlType {
        Github,
        VisualStudioTeamService,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceRegistryCredentials {
    #[serde(rename = "loginMode", default, skip_serializing_if = "Option::is_none")]
    pub login_mode: Option<source_registry_credentials::LoginMode>,
}
pub mod source_registry_credentials {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LoginMode {
        None,
        Default,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTrigger {
    #[serde(rename = "sourceRepository")]
    pub source_repository: SourceProperties,
    #[serde(rename = "sourceTriggerEvents")]
    pub source_trigger_events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<source_trigger::Status>,
    pub name: String,
}
pub mod source_trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceTriggerDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "pullRequestId", default, skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(rename = "branchName", default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceTriggerUpdateParameters {
    #[serde(rename = "sourceRepository", default, skip_serializing_if = "Option::is_none")]
    pub source_repository: Option<SourceUpdateParameters>,
    #[serde(rename = "sourceTriggerEvents", default, skip_serializing_if = "Vec::is_empty")]
    pub source_trigger_events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<source_trigger_update_parameters::Status>,
    pub name: String,
}
pub mod source_trigger_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceUpdateParameters {
    #[serde(rename = "sourceControlType", default, skip_serializing_if = "Option::is_none")]
    pub source_control_type: Option<source_update_parameters::SourceControlType>,
    #[serde(rename = "repositoryUrl", default, skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "sourceControlAuthProperties", default, skip_serializing_if = "Option::is_none")]
    pub source_control_auth_properties: Option<AuthInfoUpdateParameters>,
}
pub mod source_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SourceControlType {
        Github,
        VisualStudioTeamService,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceUploadDefinition {
    #[serde(rename = "uploadUrl", default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Task>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<task_properties::ProvisioningState>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_properties::Status>,
    pub platform: PlatformProperties,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    pub step: TaskStepProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
pub mod task_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskPropertiesUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<task_properties_update_parameters::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformUpdateParameters>,
    #[serde(rename = "agentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub agent_configuration: Option<AgentProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<TaskStepUpdateParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerUpdateParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}
pub mod task_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskRunRequest {
    #[serde(flatten)]
    pub run_request: RunRequest,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<SetValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskStepProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<task_step_properties::Type>,
    #[serde(rename = "baseImageDependencies", default, skip_serializing_if = "Vec::is_empty")]
    pub base_image_dependencies: Vec<BaseImageDependency>,
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(rename = "contextAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub context_access_token: Option<String>,
}
pub mod task_step_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Docker,
        FileTask,
        EncodedTask,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskStepUpdateParameters {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<task_step_update_parameters::Type>,
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(rename = "contextAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub context_access_token: Option<String>,
}
pub mod task_step_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Docker,
        FileTask,
        EncodedTask,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TaskPropertiesUpdateParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerProperties {
    #[serde(rename = "sourceTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub source_triggers: Vec<SourceTrigger>,
    #[serde(rename = "baseImageTrigger", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger: Option<BaseImageTrigger>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerUpdateParameters {
    #[serde(rename = "sourceTriggers", default, skip_serializing_if = "Vec::is_empty")]
    pub source_triggers: Vec<SourceTriggerUpdateParameters>,
    #[serde(rename = "baseImageTrigger", default, skip_serializing_if = "Option::is_none")]
    pub base_image_trigger: Option<BaseImageTriggerUpdateParameters>,
}
