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
pub struct WritableFrontPortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "rear_port")]
    pub rear_port: i32,
    #[serde(rename = "rear_port_position", skip_serializing_if = "Option::is_none")]
    pub rear_port_position: Option<i32>,
}

impl WritableFrontPortTemplate {
    pub fn new(
        device_type: i32,
        name: String,
        _type: Type,
        rear_port: i32,
    ) -> WritableFrontPortTemplate {
        WritableFrontPortTemplate {
            id: None,
            device_type,
            name,
            _type,
            rear_port,
            rear_port_position: None,
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
    #[serde(rename = "mrj21")]
    Mrj21,
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
