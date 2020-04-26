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
pub struct Manufacturer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "devicetype_count", skip_serializing_if = "Option::is_none")]
    pub devicetype_count: Option<i32>,
    #[serde(rename = "inventoryitem_count", skip_serializing_if = "Option::is_none")]
    pub inventoryitem_count: Option<i32>,
    #[serde(rename = "platform_count", skip_serializing_if = "Option::is_none")]
    pub platform_count: Option<i32>,
}

impl Manufacturer {
    pub fn new(name: String, slug: String) -> Manufacturer {
        Manufacturer {
            id: None,
            name,
            slug,
            devicetype_count: None,
            inventoryitem_count: None,
            platform_count: None,
        }
    }
}

