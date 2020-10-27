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
pub struct RackUnit {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "face", skip_serializing_if = "Option::is_none")]
    pub face: Option<crate::models::Face1>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::NestedDevice>,
}

impl RackUnit {
    pub fn new() -> RackUnit {
        RackUnit {
            id: None,
            name: None,
            face: None,
            device: None,
        }
    }
}
