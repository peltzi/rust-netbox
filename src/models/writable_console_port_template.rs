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
pub struct WritableConsolePortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl WritableConsolePortTemplate {
    pub fn new(device_type: i32, name: String) -> WritableConsolePortTemplate {
        WritableConsolePortTemplate {
            id: None,
            device_type,
            name,
            _type: None,
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
