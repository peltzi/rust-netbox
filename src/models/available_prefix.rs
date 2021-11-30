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
pub struct AvailablePrefix {
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<i32>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "vrf", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Box<crate::models::NestedVrf>>,
}

impl AvailablePrefix {
    pub fn new() -> AvailablePrefix {
        AvailablePrefix {
            family: None,
            prefix: None,
            vrf: None,
        }
    }
}
