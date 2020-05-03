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
pub struct PowerFeed {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "power_panel")]
    pub power_panel: crate::models::NestedPowerPanel,
    #[serde(rename = "rack", skip_serializing_if = "Option::is_none")]
    pub rack: Option<crate::models::NestedRack>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::Status>,
    #[serde(rename = "supply", skip_serializing_if = "Option::is_none")]
    pub supply: Option<crate::models::Status>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<crate::models::Status>,
    #[serde(rename = "voltage", skip_serializing_if = "Option::is_none")]
    pub voltage: Option<i32>,
    #[serde(rename = "amperage", skip_serializing_if = "Option::is_none")]
    pub amperage: Option<i32>,
    /// Maximum permissible draw (percentage)
    #[serde(rename = "max_utilization", skip_serializing_if = "Option::is_none")]
    pub max_utilization: Option<i32>,
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

impl PowerFeed {
    pub fn new(power_panel: crate::models::NestedPowerPanel, name: String) -> PowerFeed {
        PowerFeed {
            id: None,
            power_panel,
            rack: None,
            name,
            status: None,
            _type: None,
            supply: None,
            phase: None,
            voltage: None,
            amperage: None,
            max_utilization: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}
