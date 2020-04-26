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
pub struct ClusterGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "cluster_count", skip_serializing_if = "Option::is_none")]
    pub cluster_count: Option<i32>,
}

impl ClusterGroup {
    pub fn new(name: String, slug: String) -> ClusterGroup {
        ClusterGroup {
            id: None,
            name,
            slug,
            cluster_count: None,
        }
    }
}


