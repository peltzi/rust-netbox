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
pub struct DeviceInterface {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: crate::models::NestedDevice,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::Status>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lag", skip_serializing_if = "Option::is_none")]
    pub lag: Option<crate::models::NestedInterface>,
    #[serde(rename = "mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// This interface is used only for out-of-band management
    #[serde(rename = "mgmt_only", skip_serializing_if = "Option::is_none")]
    pub mgmt_only: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "connected_endpoint_type", skip_serializing_if = "Option::is_none")]
    pub connected_endpoint_type: Option<String>,
    ///          Return the appropriate serializer for the type of connected object.         
    #[serde(rename = "connected_endpoint", skip_serializing_if = "Option::is_none")]
    pub connected_endpoint: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "connection_status", skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<crate::models::ConnectionStatus>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<crate::models::NestedCable>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::models::Status>,
    #[serde(rename = "untagged_vlan", skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<crate::models::NestedVlan>,
    #[serde(rename = "tagged_vlans", skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<crate::models::NestedVlan>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "count_ipaddresses", skip_serializing_if = "Option::is_none")]
    pub count_ipaddresses: Option<String>,
}

impl DeviceInterface {
    pub fn new(device: crate::models::NestedDevice, name: String) -> DeviceInterface {
        DeviceInterface {
            id: None,
            device,
            name,
            _type: None,
            enabled: None,
            lag: None,
            mtu: None,
            mac_address: None,
            mgmt_only: None,
            description: None,
            connected_endpoint_type: None,
            connected_endpoint: None,
            connection_status: None,
            cable: None,
            mode: None,
            untagged_vlan: None,
            tagged_vlans: None,
            tags: None,
            count_ipaddresses: None,
        }
    }
}


