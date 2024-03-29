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
pub struct PowerFeed {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "power_panel")]
    pub power_panel: Box<crate::models::NestedPowerPanel>,
    #[serde(rename = "rack", skip_serializing_if = "Option::is_none")]
    pub rack: Option<Box<crate::models::NestedRack>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status3>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::models::Type3>>,
    #[serde(rename = "supply", skip_serializing_if = "Option::is_none")]
    pub supply: Option<Box<crate::models::Supply>>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<Box<crate::models::Phase>>,
    #[serde(rename = "voltage", skip_serializing_if = "Option::is_none")]
    pub voltage: Option<i32>,
    #[serde(rename = "amperage", skip_serializing_if = "Option::is_none")]
    pub amperage: Option<i32>,
    /// Maximum permissible draw (percentage)
    #[serde(rename = "max_utilization", skip_serializing_if = "Option::is_none")]
    pub max_utilization: Option<i32>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
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
    ///  Return the appropriate serializer for the type of connected object.
    #[serde(rename = "connected_endpoint", skip_serializing_if = "Option::is_none")]
    pub connected_endpoint: Option<::std::collections::HashMap<String, Value>>,
    #[serde(
        rename = "connected_endpoint_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_endpoint_type: Option<String>,
    #[serde(
        rename = "connected_endpoint_reachable",
        skip_serializing_if = "Option::is_none"
    )]
    pub connected_endpoint_reachable: Option<bool>,
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

impl PowerFeed {
    pub fn new(power_panel: crate::models::NestedPowerPanel, name: String) -> PowerFeed {
        PowerFeed {
            id: None,
            url: None,
            display: None,
            power_panel: Box::new(power_panel),
            rack: None,
            name,
            status: None,
            _type: None,
            supply: None,
            phase: None,
            voltage: None,
            amperage: None,
            max_utilization: None,
            comments: None,
            mark_connected: None,
            cable: None,
            cable_peer: None,
            cable_peer_type: None,
            connected_endpoint: None,
            connected_endpoint_type: None,
            connected_endpoint_reachable: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            _occupied: None,
        }
    }
}
