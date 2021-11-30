/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableVirtualMachineWithConfigContext {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(rename = "cluster")]
    pub cluster: i32,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<i32>,
    #[serde(rename = "primary_ip", skip_serializing_if = "Option::is_none")]
    pub primary_ip: Option<String>,
    #[serde(rename = "primary_ip4", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<i32>,
    #[serde(rename = "primary_ip6", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<i32>,
    #[serde(rename = "vcpus", skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<f32>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "local_context_data", skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "config_context", skip_serializing_if = "Option::is_none")]
    pub config_context: Option<::std::collections::HashMap<String, Value>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl WritableVirtualMachineWithConfigContext {
    pub fn new(name: String, cluster: i32) -> WritableVirtualMachineWithConfigContext {
        WritableVirtualMachineWithConfigContext {
            id: None,
            url: None,
            display: None,
            name,
            status: None,
            site: None,
            cluster,
            role: None,
            tenant: None,
            platform: None,
            primary_ip: None,
            primary_ip4: None,
            primary_ip6: None,
            vcpus: None,
            memory: None,
            disk: None,
            comments: None,
            local_context_data: None,
            tags: None,
            custom_fields: None,
            config_context: None,
            created: None,
            last_updated: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staged")]
    Staged,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}
