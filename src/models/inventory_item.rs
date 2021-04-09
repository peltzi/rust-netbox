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
pub struct InventoryItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Box<crate::models::NestedManufacturer>>,
    #[serde(rename = "part_id", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this item
    #[serde(rename = "asset_tag", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(rename = "discovered", skip_serializing_if = "Option::is_none")]
    pub discovered: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl InventoryItem {
    pub fn new(device: crate::models::NestedDevice, name: String) -> InventoryItem {
        InventoryItem {
            id: None,
            device: Box::new(device),
            parent: None,
            name,
            manufacturer: None,
            part_id: None,
            serial: None,
            asset_tag: None,
            discovered: None,
            description: None,
            tags: None,
        }
    }
}
