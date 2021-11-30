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
pub struct WritableSite {
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
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<i32>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Local facility ID or description
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<String>,
    /// 32-bit autonomous system number
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "physical_address", skip_serializing_if = "Option::is_none")]
    pub physical_address: Option<String>,
    #[serde(rename = "shipping_address", skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    /// GPS coordinate (latitude)
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// GPS coordinate (longitude)
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
    #[serde(rename = "contact_name", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(rename = "contact_phone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(rename = "contact_email", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
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
    #[serde(rename = "circuit_count", skip_serializing_if = "Option::is_none")]
    pub circuit_count: Option<i32>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
    #[serde(rename = "rack_count", skip_serializing_if = "Option::is_none")]
    pub rack_count: Option<i32>,
    #[serde(
        rename = "virtualmachine_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtualmachine_count: Option<i32>,
    #[serde(rename = "vlan_count", skip_serializing_if = "Option::is_none")]
    pub vlan_count: Option<i32>,
}

impl WritableSite {
    pub fn new(name: String, slug: String) -> WritableSite {
        WritableSite {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            status: None,
            region: None,
            group: None,
            tenant: None,
            facility: None,
            asn: None,
            time_zone: None,
            description: None,
            physical_address: None,
            shipping_address: None,
            latitude: None,
            longitude: None,
            contact_name: None,
            contact_phone: None,
            contact_email: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            circuit_count: None,
            device_count: None,
            prefix_count: None,
            rack_count: None,
            virtualmachine_count: None,
            vlan_count: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "decommissioning")]
    Decommissioning,
    #[serde(rename = "retired")]
    Retired,
}
