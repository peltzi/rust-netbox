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
pub struct WritableDeviceType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "manufacturer")]
    pub manufacturer: i32,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "slug")]
    pub slug: String,
    /// Discrete part number (optional)
    #[serde(rename = "part_number", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "u_height", skip_serializing_if = "Option::is_none")]
    pub u_height: Option<i32>,
    /// Device consumes both front and rear rack faces
    #[serde(rename = "is_full_depth", skip_serializing_if = "Option::is_none")]
    pub is_full_depth: Option<bool>,
    /// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
    #[serde(rename = "subdevice_role", skip_serializing_if = "Option::is_none")]
    pub subdevice_role: Option<SubdeviceRole>,
    #[serde(rename = "front_image", skip_serializing_if = "Option::is_none")]
    pub front_image: Option<String>,
    #[serde(rename = "rear_image", skip_serializing_if = "Option::is_none")]
    pub rear_image: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
}

impl WritableDeviceType {
    pub fn new(manufacturer: i32, model: String, slug: String) -> WritableDeviceType {
        WritableDeviceType {
            id: None,
            url: None,
            display: None,
            manufacturer,
            model,
            slug,
            part_number: None,
            u_height: None,
            is_full_depth: None,
            subdevice_role: None,
            front_image: None,
            rear_image: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            device_count: None,
        }
    }
}

/// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubdeviceRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
}
