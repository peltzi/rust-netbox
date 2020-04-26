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
pub struct VlanGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<crate::models::NestedSite>,
    #[serde(rename = "vlan_count", skip_serializing_if = "Option::is_none")]
    pub vlan_count: Option<i32>,
}

impl VlanGroup {
    pub fn new(name: String, slug: String) -> VlanGroup {
        VlanGroup {
            id: None,
            name,
            slug,
            site: None,
            vlan_count: None,
        }
    }
}


