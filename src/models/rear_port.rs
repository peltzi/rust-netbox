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
pub struct RearPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub _type: Box<crate::models::Type1>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    ///  Return the appropriate serializer for the cable termination model.
    #[serde(rename = "cable_peer", skip_serializing_if = "Option::is_none")]
    pub cable_peer: Option<::std::collections::HashMap<String, Value>>,
    #[serde(rename = "cable_peer_type", skip_serializing_if = "Option::is_none")]
    pub cable_peer_type: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl RearPort {
    pub fn new(
        device: crate::models::NestedDevice,
        name: String,
        _type: crate::models::Type1,
    ) -> RearPort {
        RearPort {
            id: None,
            url: None,
            display: None,
            device: Box::new(device),
            name,
            label: None,
            _type: Box::new(_type),
            color: None,
            positions: None,
            description: None,
            mark_connected: None,
            cable: None,
            cable_peer: None,
            cable_peer_type: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            _occupied: None,
        }
    }
}
