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
pub struct NestedCircuit {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "cid")]
    pub cid: String,
}

impl NestedCircuit {
    pub fn new(cid: String) -> NestedCircuit {
        NestedCircuit {
            id: None,
            url: None,
            cid,
        }
    }
}

