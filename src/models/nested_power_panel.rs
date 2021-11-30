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
pub struct NestedPowerPanel {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "powerfeed_count", skip_serializing_if = "Option::is_none")]
    pub powerfeed_count: Option<i32>,
}

impl NestedPowerPanel {
    pub fn new(name: String) -> NestedPowerPanel {
        NestedPowerPanel {
            id: None,
            url: None,
            display: None,
            name,
            powerfeed_count: None,
        }
    }
}
