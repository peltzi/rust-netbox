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
pub struct Status6 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Status6 {
    pub fn new(label: Label, value: Value) -> Status6 {
        Status6 { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "Deprecated")]
    Deprecated,
    #[serde(rename = "DHCP")]
    DHCP,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "dhcp")]
    Dhcp,
}
