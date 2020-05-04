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
pub struct Status1 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Status1 {
    pub fn new(label: Label, value: Value) -> Status1 {
        Status1 { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Connected")]
    Connected,
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "Decommissioning")]
    Decommissioning,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}
