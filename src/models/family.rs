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
pub struct Family {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: i32,
}

impl Family {
    pub fn new(label: Label, value: i32) -> Family {
        Family { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "IPv4")]
    IPv4,
    #[serde(rename = "IPv6")]
    IPv6,
}
