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
pub struct ExportTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "template_language", skip_serializing_if = "Option::is_none")]
    pub template_language: Option<crate::models::Status>,
    /// The list of objects being exported is passed as a context variable named <code>queryset</code>.
    #[serde(rename = "template_code")]
    pub template_code: String,
    /// Defaults to <code>text/plain</code>
    #[serde(rename = "mime_type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Extension to append to the rendered filename
    #[serde(rename = "file_extension", skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
}

impl ExportTemplate {
    pub fn new(content_type: String, name: String, template_code: String) -> ExportTemplate {
        ExportTemplate {
            id: None,
            content_type,
            name,
            description: None,
            template_language: None,
            template_code,
            mime_type: None,
            file_extension: None,
        }
    }
}

