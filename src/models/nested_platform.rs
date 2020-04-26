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
pub struct NestedPlatform {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "virtualmachine_count", skip_serializing_if = "Option::is_none")]
    pub virtualmachine_count: Option<i32>,
}

impl NestedPlatform {
    pub fn new(name: String, slug: String) -> NestedPlatform {
        NestedPlatform {
            id: None,
            url: None,
            name,
            slug,
            device_count: None,
            virtualmachine_count: None,
        }
    }
}

