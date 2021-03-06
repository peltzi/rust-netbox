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
pub struct NestedVrf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rd", skip_serializing_if = "Option::is_none")]
    pub rd: Option<String>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
}

impl NestedVrf {
    pub fn new(name: String) -> NestedVrf {
        NestedVrf {
            id: None,
            url: None,
            name,
            rd: None,
            prefix_count: None,
        }
    }
}
