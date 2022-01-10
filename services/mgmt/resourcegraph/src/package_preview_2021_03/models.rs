#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ColumnDataType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColumnDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeInterval {
    pub start: String,
    pub end: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Facet {
    pub expression: String,
    #[serde(rename = "resultType")]
    pub result_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetError {
    #[serde(flatten)]
    pub facet: Facet,
    pub errors: Vec<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetRequest {
    pub expression: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<FacetRequestOptions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetRequestOptions {
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<facet_request_options::SortOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
}
pub mod facet_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl Default for SortOrder {
        fn default() -> Self {
            Self::Desc
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetResult {
    #[serde(flatten)]
    pub facet: Facet,
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    pub count: i32,
    pub data: serde_json::Value,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(rename = "managementGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub management_groups: Vec<String>,
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<QueryRequestOptions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<FacetRequest>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryRequestOptions {
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<query_request_options::ResultFormat>,
    #[serde(rename = "allowPartialScopes", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_scopes: Option<bool>,
}
pub mod query_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
    impl Default for ResultFormat {
        fn default() -> Self {
            Self::ObjectArray
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
    pub count: i64,
    #[serde(rename = "resultTruncated")]
    pub result_truncated: query_response::ResultTruncated,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    pub data: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<Facet>,
}
pub mod query_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultTruncated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangeData {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "changeId")]
    pub change_id: String,
    #[serde(rename = "beforeSnapshot")]
    pub before_snapshot: serde_json::Value,
    #[serde(rename = "afterSnapshot")]
    pub after_snapshot: serde_json::Value,
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<resource_change_data::ChangeType>,
    #[serde(rename = "propertyChanges", default, skip_serializing_if = "Vec::is_empty")]
    pub property_changes: Vec<ResourcePropertyChange>,
}
pub mod resource_change_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Create,
        Update,
        Delete,
    }
}
pub type ResourceChangeDataList = Vec<ResourceChangeData>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangeDetailsRequestParameters {
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
    #[serde(rename = "changeIds")]
    pub change_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceChangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<ResourceChangeData>,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceChangesRequestParameters {
    #[serde(rename = "resourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_ids: Vec<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    pub interval: serde_json::Value,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "fetchPropertyChanges", default, skip_serializing_if = "Option::is_none")]
    pub fetch_property_changes: Option<bool>,
    #[serde(rename = "fetchSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub fetch_snapshots: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePropertyChange {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(rename = "beforeValue", default, skip_serializing_if = "Option::is_none")]
    pub before_value: Option<String>,
    #[serde(rename = "afterValue", default, skip_serializing_if = "Option::is_none")]
    pub after_value: Option<String>,
    #[serde(rename = "changeCategory")]
    pub change_category: resource_property_change::ChangeCategory,
    #[serde(rename = "propertyChangeType")]
    pub property_change_type: resource_property_change::PropertyChangeType,
}
pub mod resource_property_change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeCategory {
        User,
        System,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyChangeType {
        Insert,
        Update,
        Remove,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSnapshotData {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesHistoryRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ResourcesHistoryRequestOptions>,
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesHistoryRequestOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<DateTimeInterval>,
    #[serde(rename = "$top", default, skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    #[serde(rename = "$skip", default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(rename = "$skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<resources_history_request_options::ResultFormat>,
}
pub mod resources_history_request_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "objectArray")]
        ObjectArray,
    }
}
pub type Row = Vec<serde_json::Value>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}
