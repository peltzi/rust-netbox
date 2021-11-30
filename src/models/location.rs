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
pub struct Location {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "site")]
    pub site: Option<Box<crate::models::NestedSite>>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::NestedLocation>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "rack_count", skip_serializing_if = "Option::is_none")]
    pub rack_count: Option<i32>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "_depth", skip_serializing_if = "Option::is_none")]
    pub _depth: Option<i32>,
}

impl Location {
    pub fn new(name: String, slug: String, site: Option<crate::models::NestedSite>) -> Location {
        let nested_site = match site {
            Some(s) => Some(Box::new(s)),
            None => None,
        };

        Location {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            site: nested_site,
            parent: None,
            description: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            rack_count: None,
            device_count: None,
            _depth: None,
        }
    }
}
