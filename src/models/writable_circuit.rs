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
pub struct WritableCircuit {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    #[serde(rename = "install_date", skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
    #[serde(rename = "commit_rate", skip_serializing_if = "Option::is_none")]
    pub commit_rate: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termination_a", skip_serializing_if = "Option::is_none")]
    pub termination_a: Option<String>,
    #[serde(rename = "termination_z", skip_serializing_if = "Option::is_none")]
    pub termination_z: Option<String>,
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
}

impl WritableCircuit {
    pub fn new(cid: String, provider: i32, _type: i32) -> WritableCircuit {
        WritableCircuit {
            id: None,
            cid,
            provider,
            _type,
            status: None,
            tenant: None,
            install_date: None,
            commit_rate: None,
            description: None,
            termination_a: None,
            termination_z: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "deprovisioning")]
    Deprovisioning,
    #[serde(rename = "decommissioned")]
    Decommissioned,
}

