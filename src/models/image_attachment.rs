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
pub struct ImageAttachment {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "object_id")]
    pub object_id: i32,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "image_height")]
    pub image_height: i32,
    #[serde(rename = "image_width")]
    pub image_width: i32,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

impl ImageAttachment {
    pub fn new(
        content_type: String,
        object_id: i32,
        image_height: i32,
        image_width: i32,
    ) -> ImageAttachment {
        ImageAttachment {
            id: None,
            content_type,
            object_id,
            parent: None,
            name: None,
            image: None,
            image_height,
            image_width,
            created: None,
        }
    }
}
