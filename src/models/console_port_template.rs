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
pub struct ConsolePortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: crate::models::NestedDeviceType,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ModelType>,
}

impl ConsolePortTemplate {
    pub fn new(device_type: crate::models::NestedDeviceType, name: String) -> ConsolePortTemplate {
        ConsolePortTemplate {
            id: None,
            device_type,
            name,
            _type: None,
        }
    }
}
