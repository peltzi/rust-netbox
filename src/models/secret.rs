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
pub struct Secret {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: crate::models::NestedDevice,
    #[serde(rename = "role")]
    pub role: crate::models::NestedSecretRole,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "plaintext")]
    pub plaintext: String,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl Secret {
    pub fn new(device: crate::models::NestedDevice, role: crate::models::NestedSecretRole, plaintext: String) -> Secret {
        Secret {
            id: None,
            device,
            role,
            name: None,
            plaintext,
            hash: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}


