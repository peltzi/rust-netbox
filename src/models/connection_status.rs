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
pub struct ConnectionStatus {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "value")]
    pub value: Option<bool>,
}

impl ConnectionStatus {
    pub fn new(label: String, value: Option<bool>) -> ConnectionStatus {
        ConnectionStatus { label, value }
    }
}
