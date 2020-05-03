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
pub struct ConsolePort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: crate::models::NestedDevice,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::Status>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "connected_endpoint_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_endpoint_type: Option<String>,
    ///          Return the appropriate serializer for the type of connected object.         
    #[serde(rename = "connected_endpoint", skip_serializing_if = "Option::is_none")]
    pub connected_endpoint: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "connection_status", skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<crate::models::ConnectionStatus>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<crate::models::NestedCable>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ConsolePort {
    pub fn new(device: crate::models::NestedDevice, name: String) -> ConsolePort {
        ConsolePort {
            id: None,
            device,
            name,
            _type: None,
            description: None,
            connected_endpoint_type: None,
            connected_endpoint: None,
            connection_status: None,
            cable: None,
            tags: None,
        }
    }
}
