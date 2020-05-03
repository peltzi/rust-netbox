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
pub struct PowerPortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: crate::models::NestedDeviceType,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::Status>,
    /// Maximum power draw (watts)
    #[serde(rename = "maximum_draw", skip_serializing_if = "Option::is_none")]
    pub maximum_draw: Option<i32>,
    /// Allocated power draw (watts)
    #[serde(rename = "allocated_draw", skip_serializing_if = "Option::is_none")]
    pub allocated_draw: Option<i32>,
}

impl PowerPortTemplate {
    pub fn new(device_type: crate::models::NestedDeviceType, name: String) -> PowerPortTemplate {
        PowerPortTemplate {
            id: None,
            device_type,
            name,
            _type: None,
            maximum_draw: None,
            allocated_draw: None,
        }
    }
}
