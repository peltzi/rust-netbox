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
pub struct InlineResponse20018 {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::Interface>,
}

impl InlineResponse20018 {
    pub fn new(count: i32, results: Vec<crate::models::Interface>) -> InlineResponse20018 {
        InlineResponse20018 {
            count,
            next: None,
            previous: None,
            results,
        }
    }
}
