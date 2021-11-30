/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Status {
    pub fn new(label: Label, value: Value) -> Status {
        Status { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "Provisioning")]
    Provisioning,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Offline")]
    Offline,
    #[serde(rename = "Deprovisioning")]
    Deprovisioning,
    #[serde(rename = "Decommissioned")]
    Decommissioned,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "deprovisioning")]
    Deprovisioning,
    #[serde(rename = "decommissioned")]
    Decommissioned,
}
