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
pub struct Rir {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    /// IP space managed by this RIR is considered private
    #[serde(rename = "is_private", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(rename = "aggregate_count", skip_serializing_if = "Option::is_none")]
    pub aggregate_count: Option<i32>,
}

impl Rir {
    pub fn new(name: String, slug: String) -> Rir {
        Rir {
            id: None,
            name,
            slug,
            is_private: None,
            aggregate_count: None,
        }
    }
}
