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
pub struct NestedPowerPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::NestedDevice>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<i32>,
    #[serde(rename = "connection_status", skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<crate::models::ConnectionStatus>,
}

impl NestedPowerPort {
    pub fn new(name: String) -> NestedPowerPort {
        NestedPowerPort {
            id: None,
            url: None,
            device: None,
            name,
            cable: None,
            connection_status: None,
        }
    }
}
