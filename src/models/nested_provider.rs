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
pub struct NestedProvider {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "circuit_count", skip_serializing_if = "Option::is_none")]
    pub circuit_count: Option<i32>,
}

impl NestedProvider {
    pub fn new(name: String, slug: String) -> NestedProvider {
        NestedProvider {
            id: None,
            url: None,
            name,
            slug,
            circuit_count: None,
        }
    }
}


