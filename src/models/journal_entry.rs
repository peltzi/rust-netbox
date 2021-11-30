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
pub struct JournalEntry {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "assigned_object_type")]
    pub assigned_object_type: String,
    #[serde(rename = "assigned_object_id")]
    pub assigned_object_id: i32,
    #[serde(rename = "assigned_object", skip_serializing_if = "Option::is_none")]
    pub assigned_object: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i32>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<crate::models::Kind>>,
    #[serde(rename = "comments")]
    pub comments: String,
}

impl JournalEntry {
    pub fn new(
        assigned_object_type: String,
        assigned_object_id: i32,
        comments: String,
    ) -> JournalEntry {
        JournalEntry {
            id: None,
            url: None,
            display: None,
            assigned_object_type,
            assigned_object_id,
            assigned_object: None,
            created: None,
            created_by: None,
            kind: None,
            comments,
        }
    }
}
