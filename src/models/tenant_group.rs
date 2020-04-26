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
pub struct TenantGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "tenant_count", skip_serializing_if = "Option::is_none")]
    pub tenant_count: Option<i32>,
}

impl TenantGroup {
    pub fn new(name: String, slug: String) -> TenantGroup {
        TenantGroup {
            id: None,
            name,
            slug,
            tenant_count: None,
        }
    }
}


