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
pub struct NestedDeviceType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Box<crate::models::NestedManufacturer>>,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
}

impl NestedDeviceType {
    pub fn new(model: String, slug: String) -> NestedDeviceType {
        NestedDeviceType {
            id: None,
            url: None,
            display: None,
            manufacturer: None,
            model,
            slug,
            device_count: None,
        }
    }
}
