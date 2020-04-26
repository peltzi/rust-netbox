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
pub struct WritableCluster {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
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
    #[serde(rename = "virtualmachine_count", skip_serializing_if = "Option::is_none")]
    pub virtualmachine_count: Option<i32>,
}

impl WritableCluster {
    pub fn new(name: String, _type: i32) -> WritableCluster {
        WritableCluster {
            id: None,
            name,
            _type,
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

