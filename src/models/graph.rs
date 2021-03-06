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
pub struct Graph {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "template_language", skip_serializing_if = "Option::is_none")]
    pub template_language: Option<TemplateLanguage>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl Graph {
    pub fn new(_type: String, name: String, source: String) -> Graph {
        Graph {
            id: None,
            _type,
            weight: None,
            name,
            template_language: None,
            source,
            link: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateLanguage {
    #[serde(rename = "django")]
    Django,
    #[serde(rename = "jinja2")]
    Jinja2,
}
