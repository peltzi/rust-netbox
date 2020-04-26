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
pub struct IpAddress {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<crate::models::Family>,
    /// IPv4 or IPv6 address (with mask)
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "vrf", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<crate::models::NestedVrf>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<crate::models::NestedTenant>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::Status>,
    #[serde(rename = "interface", skip_serializing_if = "Option::is_none")]
    pub interface: Option<crate::models::IpAddressInterface>,
    #[serde(rename = "nat_inside", skip_serializing_if = "Option::is_none")]
    pub nat_inside: Option<crate::models::NestedIpAddress>,
    #[serde(rename = "nat_outside", skip_serializing_if = "Option::is_none")]
    pub nat_outside: Option<crate::models::NestedIpAddress>,
    /// Hostname or FQDN (not case-sensitive)
    #[serde(rename = "dns_name", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl IpAddress {
    pub fn new(address: String) -> IpAddress {
        IpAddress {
            id: None,
            family: None,
            address,
            vrf: None,
            tenant: None,
            status: None,
            role: None,
            interface: None,
            nat_inside: None,
            nat_outside: None,
            dns_name: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}


