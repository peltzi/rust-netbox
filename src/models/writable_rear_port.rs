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
pub struct WritableRearPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<crate::models::NestedCable>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WritableRearPort {
    pub fn new(device: i32, name: String, _type: Type) -> WritableRearPort {
        WritableRearPort {
            id: None,
            device,
            name,
            _type,
            positions: None,
            description: None,
            cable: None,
            tags: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "8p8c")]
    _8p8c,
    #[serde(rename = "110-punch")]
    _110Punch,
    #[serde(rename = "bnc")]
    Bnc,
    #[serde(rename = "fc")]
    Fc,
    #[serde(rename = "lc")]
    Lc,
    #[serde(rename = "lc-apc")]
    LcApc,
    #[serde(rename = "lsh")]
    Lsh,
    #[serde(rename = "lsh-apc")]
    LshApc,
    #[serde(rename = "mpo")]
    Mpo,
    #[serde(rename = "mtrj")]
    Mtrj,
    #[serde(rename = "sc")]
    Sc,
    #[serde(rename = "sc-apc")]
    ScApc,
    #[serde(rename = "st")]
    St,
}
