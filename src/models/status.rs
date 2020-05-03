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
pub struct Status {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl Status {
    pub fn new(label: String, value: String) -> Status {
        Status { label, value }
    }
}
