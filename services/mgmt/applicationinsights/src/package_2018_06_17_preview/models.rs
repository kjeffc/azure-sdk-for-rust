#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Error Field contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorFieldContract {
    #[doc = "Property level error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of property-level error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorFieldContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CDN REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Cdn"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list CDN operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of CDN operations supported by the CDN resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name. This is GUID value. The display name should be assigned within properties field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<resource::Kind>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            kind: None,
            location,
            tags: None,
        }
    }
}
pub mod resource {
    use super::*;
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "shared")]
        Shared,
    }
}
#[doc = "An Application Insights workbook definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties that contain a workbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookProperties>,
}
impl Workbook {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Error message body that will indicate why the operation failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookError {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of invalid fields send in request, in case of validation error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorFieldContract>,
}
impl WorkbookError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties that contain a workbook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookProperties {
    #[doc = "The user-defined name (display name) of the workbook."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Configuration of this particular workbook. Configuration data is a string containing valid JSON"]
    #[serde(rename = "serializedData")]
    pub serialized_data: String,
    #[doc = "Date and time in UTC of the last modification that was made to this workbook definition."]
    #[serde(rename = "timeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[doc = "Workbook category, as defined by the user at creation time."]
    pub category: String,
    #[doc = "Workbook version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A list of 0 or more tags that are associated with this workbook definition"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "Unique user id of the specific user that owns this workbook."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "ResourceId for a source resource."]
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
impl WorkbookProperties {
    pub fn new(display_name: String, serialized_data: String, category: String) -> Self {
        Self {
            display_name,
            serialized_data,
            time_modified: None,
            category,
            version: None,
            tags: Vec::new(),
            user_id: None,
            source_id: None,
        }
    }
}
#[doc = "Properties that contain a workbook for PATCH operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookPropertiesUpdateParameters {
    #[doc = "The user-defined name (display name) of the workbook."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Configuration of this particular workbook. Configuration data is a string containing valid JSON"]
    #[serde(rename = "serializedData", default, skip_serializing_if = "Option::is_none")]
    pub serialized_data: Option<String>,
    #[doc = "Workbook category, as defined by the user at creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "A list of 0 or more tags that are associated with this workbook definition"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl WorkbookPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters that can be provided when updating workbook properties properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbookUpdateParameters {
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<workbook_update_parameters::Kind>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties that contain a workbook for PATCH operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookPropertiesUpdateParameters>,
}
impl WorkbookUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workbook_update_parameters {
    use super::*;
    #[doc = "The kind of workbook. Choices are user and shared."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "shared")]
        Shared,
    }
}
#[doc = "Workbook list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkbooksListResult {
    #[doc = "An array of workbooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workbook>,
}
impl WorkbooksListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
