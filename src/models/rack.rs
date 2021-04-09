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
pub struct Rack {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "facility_id", skip_serializing_if = "Option::is_none")]
    pub facility_id: Option<String>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "site")]
    pub site: Box<crate::models::NestedSite>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::NestedRackGroup>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<crate::models::NestedTenant>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status4>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::NestedRackRole>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this rack
    #[serde(rename = "asset_tag", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::models::Type6>>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<Box<crate::models::Width>>,
    #[serde(rename = "u_height", skip_serializing_if = "Option::is_none")]
    pub u_height: Option<i32>,
    /// Units are numbered top-to-bottom
    #[serde(rename = "desc_units", skip_serializing_if = "Option::is_none")]
    pub desc_units: Option<bool>,
    #[serde(rename = "outer_width", skip_serializing_if = "Option::is_none")]
    pub outer_width: Option<i32>,
    #[serde(rename = "outer_depth", skip_serializing_if = "Option::is_none")]
    pub outer_depth: Option<i32>,
    #[serde(rename = "outer_unit", skip_serializing_if = "Option::is_none")]
    pub outer_unit: Option<Box<crate::models::OuterUnit>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "powerfeed_count", skip_serializing_if = "Option::is_none")]
    pub powerfeed_count: Option<i32>,
}

impl Rack {
    pub fn new(name: String, site: crate::models::NestedSite) -> Rack {
        Rack {
            id: None,
            name,
            facility_id: None,
            display_name: None,
            site: Box::new(site),
            group: None,
            tenant: None,
            status: None,
            role: None,
            serial: None,
            asset_tag: None,
            _type: None,
            width: None,
            u_height: None,
            desc_units: None,
            outer_width: None,
            outer_depth: None,
            outer_unit: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            device_count: None,
            powerfeed_count: None,
        }
    }
}
