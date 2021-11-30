/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualChassis {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "master", skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "member_count", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
}

impl VirtualChassis {
    pub fn new(name: String) -> VirtualChassis {
        VirtualChassis {
            id: None,
            url: None,
            display: None,
            name,
            domain: None,
            master: None,
            tags: None,
            custom_fields: None,
            member_count: None,
        }
    }
}
