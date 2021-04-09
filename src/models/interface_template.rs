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
pub struct InterfaceTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: Box<crate::models::NestedDeviceType>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Box<crate::models::Type2>,
    #[serde(rename = "mgmt_only", skip_serializing_if = "Option::is_none")]
    pub mgmt_only: Option<bool>,
}

impl InterfaceTemplate {
    pub fn new(
        device_type: crate::models::NestedDeviceType,
        name: String,
        _type: crate::models::Type2,
    ) -> InterfaceTemplate {
        InterfaceTemplate {
            id: None,
            device_type: Box::new(device_type),
            name,
            _type: Box::new(_type),
            mgmt_only: None,
        }
    }
}
