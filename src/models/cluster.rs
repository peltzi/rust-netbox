/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 2.7
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Box<crate::models::NestedClusterType>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::NestedClusterGroup>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<crate::models::NestedTenant>>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<Box<crate::models::NestedSite>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(
        rename = "virtualmachine_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtualmachine_count: Option<i32>,
}

impl Cluster {
    pub fn new(name: String, _type: crate::models::NestedClusterType) -> Cluster {
        Cluster {
            id: None,
            name,
            _type: Box::new(_type),
            group: None,
            tenant: None,
            site: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            device_count: None,
            virtualmachine_count: None,
        }
    }
}
