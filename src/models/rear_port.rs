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
pub struct RearPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: crate::models::NestedDevice,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: crate::models::Status,
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<crate::models::NestedCable>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl RearPort {
    pub fn new(
        device: crate::models::NestedDevice,
        name: String,
        _type: crate::models::Status,
    ) -> RearPort {
        RearPort {
            id: None,
            device,
            name,
            _type,
            positions: None,
            description: None,
            cable: None,
            tags: None,
        }
    }
}
