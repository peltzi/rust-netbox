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
pub struct Webhook {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "content_types")]
    pub content_types: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// Call this webhook when a matching object is created.
    #[serde(rename = "type_create", skip_serializing_if = "Option::is_none")]
    pub type_create: Option<bool>,
    /// Call this webhook when a matching object is updated.
    #[serde(rename = "type_update", skip_serializing_if = "Option::is_none")]
    pub type_update: Option<bool>,
    /// Call this webhook when a matching object is deleted.
    #[serde(rename = "type_delete", skip_serializing_if = "Option::is_none")]
    pub type_delete: Option<bool>,
    /// A POST will be sent to this URL when the webhook is called.
    #[serde(rename = "payload_url")]
    pub payload_url: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "http_method", skip_serializing_if = "Option::is_none")]
    pub http_method: Option<HttpMethod>,
    /// The complete list of official content types is available <a href=\"https://www.iana.org/assignments/media-types/media-types.xhtml\">here</a>.
    #[serde(rename = "http_content_type", skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
    #[serde(rename = "additional_headers", skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// When provided, the request will include a 'X-Hook-Signature' header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Enable SSL certificate verification. Disable with caution!
    #[serde(rename = "ssl_verification", skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
    #[serde(rename = "ca_file_path", skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<String>,
}

impl Webhook {
    pub fn new(content_types: Vec<String>, name: String, payload_url: String) -> Webhook {
        Webhook {
            id: None,
            url: None,
            display: None,
            content_types,
            name,
            type_create: None,
            type_update: None,
            type_delete: None,
            payload_url,
            enabled: None,
            http_method: None,
            http_content_type: None,
            additional_headers: None,
            body_template: None,
            secret: None,
            ssl_verification: None,
            ca_file_path: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
}