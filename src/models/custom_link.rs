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
pub struct CustomLink {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Jinja2 template code for link text
    #[serde(rename = "link_text")]
    pub link_text: String,
    /// Jinja2 template code for link URL
    #[serde(rename = "link_url")]
    pub link_url: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Links with the same group will appear as a dropdown menu
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The class of the first link in a group will be used for the dropdown button
    #[serde(rename = "button_class", skip_serializing_if = "Option::is_none")]
    pub button_class: Option<ButtonClass>,
    /// Force link to open in a new window
    #[serde(rename = "new_window", skip_serializing_if = "Option::is_none")]
    pub new_window: Option<bool>,
}

impl CustomLink {
    pub fn new(
        content_type: String,
        name: String,
        link_text: String,
        link_url: String,
    ) -> CustomLink {
        CustomLink {
            id: None,
            url: None,
            display: None,
            content_type,
            name,
            link_text,
            link_url,
            weight: None,
            group_name: None,
            button_class: None,
            new_window: None,
        }
    }
}

/// The class of the first link in a group will be used for the dropdown button
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ButtonClass {
    #[serde(rename = "outline-dark")]
    OutlineDark,
    #[serde(rename = "ghost-dark")]
    GhostDark,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "indigo")]
    Indigo,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "secondary")]
    Secondary,
}