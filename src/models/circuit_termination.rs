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
pub struct CircuitTermination {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "circuit")]
    pub circuit: Box<crate::models::NestedCircuit>,
    #[serde(rename = "term_side")]
    pub term_side: TermSide,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<Box<crate::models::NestedSite>>,
    #[serde(rename = "provider_network", skip_serializing_if = "Option::is_none")]
    pub provider_network: Option<Box<crate::models::NestedProviderNetwork>>,
    #[serde(rename = "port_speed", skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<i32>,
    /// Upstream speed, if different from port speed
    #[serde(rename = "upstream_speed", skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<i32>,
    #[serde(rename = "xconnect_id", skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
    #[serde(rename = "pp_info", skip_serializing_if = "Option::is_none")]
    pub pp_info: Option<String>,
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
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl CircuitTermination {
    pub fn new(circuit: crate::models::NestedCircuit, term_side: TermSide) -> CircuitTermination {
        CircuitTermination {
            id: None,
            url: None,
            display: None,
            circuit: Box::new(circuit),
            term_side,
            site: None,
            provider_network: None,
            port_speed: None,
            upstream_speed: None,
            xconnect_id: None,
            pp_info: None,
            description: None,
            mark_connected: None,
            cable: None,
            cable_peer: None,
            cable_peer_type: None,
            _occupied: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TermSide {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "Z")]
    Z,
}
