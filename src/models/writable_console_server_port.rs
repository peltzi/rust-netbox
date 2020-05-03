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
pub struct WritableConsoleServerPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
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
    pub connection_status: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<crate::models::NestedCable>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WritableConsoleServerPort {
    pub fn new(device: i32, name: String) -> WritableConsoleServerPort {
        WritableConsoleServerPort {
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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "de-9")]
    De9,
    #[serde(rename = "db-25")]
    Db25,
    #[serde(rename = "rj-11")]
    Rj11,
    #[serde(rename = "rj-12")]
    Rj12,
    #[serde(rename = "rj-45")]
    Rj45,
    #[serde(rename = "usb-a")]
    UsbA,
    #[serde(rename = "usb-b")]
    UsbB,
    #[serde(rename = "usb-c")]
    UsbC,
    #[serde(rename = "usb-mini-a")]
    UsbMiniA,
    #[serde(rename = "usb-mini-b")]
    UsbMiniB,
    #[serde(rename = "usb-micro-a")]
    UsbMicroA,
    #[serde(rename = "usb-micro-b")]
    UsbMicroB,
    #[serde(rename = "other")]
    Other,
}
