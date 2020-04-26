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
pub struct WritableVrf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rd", skip_serializing_if = "Option::is_none")]
    pub rd: Option<String>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Prevent duplicate prefixes/IP addresses within this VRF
    #[serde(rename = "enforce_unique", skip_serializing_if = "Option::is_none")]
    pub enforce_unique: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "ipaddress_count", skip_serializing_if = "Option::is_none")]
    pub ipaddress_count: Option<i32>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
}

impl WritableVrf {
    pub fn new(name: String) -> WritableVrf {
        WritableVrf {
            id: None,
            name,
            rd: None,
            tenant: None,
            enforce_unique: None,
            description: None,
            tags: None,
            display_name: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            ipaddress_count: None,
            prefix_count: None,
        }
    }
}


