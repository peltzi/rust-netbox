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
pub struct Type6 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type6 {
    pub fn new(label: Label, value: Value) -> Type6 {
        Type6 { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "2-post frame")]
    _2PostFrame,
    #[serde(rename = "4-post frame")]
    _4PostFrame,
    #[serde(rename = "4-post cabinet")]
    _4PostCabinet,
    #[serde(rename = "Wall-mounted frame")]
    WallMountedFrame,
    #[serde(rename = "Wall-mounted cabinet")]
    WallMountedCabinet,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "2-post-frame")]
    _2PostFrame,
    #[serde(rename = "4-post-frame")]
    _4PostFrame,
    #[serde(rename = "4-post-cabinet")]
    _4PostCabinet,
    #[serde(rename = "wall-frame")]
    WallFrame,
    #[serde(rename = "wall-cabinet")]
    WallCabinet,
}
