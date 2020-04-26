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
pub struct CircuitCircuitTermination {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "site")]
    pub site: crate::models::NestedSite,
    #[serde(rename = "connected_endpoint")]
    pub connected_endpoint: crate::models::NestedInterface,
    #[serde(rename = "port_speed")]
    pub port_speed: i32,
    /// Upstream speed, if different from port speed
    #[serde(rename = "upstream_speed", skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<i32>,
    #[serde(rename = "xconnect_id", skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
}

impl CircuitCircuitTermination {
    pub fn new(site: crate::models::NestedSite, connected_endpoint: crate::models::NestedInterface, port_speed: i32) -> CircuitCircuitTermination {
        CircuitCircuitTermination {
            id: None,
            url: None,
            site,
            connected_endpoint,
            port_speed,
            upstream_speed: None,
            xconnect_id: None,
        }
    }
}


