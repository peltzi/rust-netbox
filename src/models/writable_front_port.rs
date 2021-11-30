/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableFrontPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "rear_port")]
    pub rear_port: i32,
    #[serde(rename = "rear_port_position", skip_serializing_if = "Option::is_none")]
    pub rear_port_position: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    ///  Return the appropriate serializer for the cable termination model.
    #[serde(rename = "cable_peer", skip_serializing_if = "Option::is_none")]
    pub cable_peer: Option<::std::collections::HashMap<String, Value>>,
    #[serde(rename = "cable_peer_type", skip_serializing_if = "Option::is_none")]
    pub cable_peer_type: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl WritableFrontPort {
    pub fn new(device: i32, name: String, _type: Type, rear_port: i32) -> WritableFrontPort {
        WritableFrontPort {
            id: None,
            url: None,
            display: None,
            device,
            name,
            label: None,
            _type,
            color: None,
            rear_port,
            rear_port_position: None,
            description: None,
            mark_connected: None,
            cable: None,
            cable_peer: None,
            cable_peer_type: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            _occupied: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "8p8c")]
    _8p8c,
    #[serde(rename = "8p6c")]
    _8p6c,
    #[serde(rename = "8p4c")]
    _8p4c,
    #[serde(rename = "8p2c")]
    _8p2c,
    #[serde(rename = "6p6c")]
    _6p6c,
    #[serde(rename = "6p4c")]
    _6p4c,
    #[serde(rename = "6p2c")]
    _6p2c,
    #[serde(rename = "4p4c")]
    _4p4c,
    #[serde(rename = "4p2c")]
    _4p2c,
    #[serde(rename = "gg45")]
    Gg45,
    #[serde(rename = "tera-4p")]
    Tera4p,
    #[serde(rename = "tera-2p")]
    Tera2p,
    #[serde(rename = "tera-1p")]
    Tera1p,
    #[serde(rename = "110-punch")]
    _110Punch,
    #[serde(rename = "bnc")]
    Bnc,
    #[serde(rename = "f")]
    F,
    #[serde(rename = "n")]
    N,
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
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "sma-905")]
    Sma905,
    #[serde(rename = "sma-906")]
    Sma906,
    #[serde(rename = "urm-p2")]
    UrmP2,
    #[serde(rename = "urm-p4")]
    UrmP4,
    #[serde(rename = "urm-p8")]
    UrmP8,
    #[serde(rename = "splice")]
    Splice,
}
