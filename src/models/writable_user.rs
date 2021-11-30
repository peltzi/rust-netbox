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
pub struct WritableUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Designates whether the user can log into this admin site.
    #[serde(rename = "is_staff", skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    /// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "date_joined", skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// The groups this user belongs to. A user will get all permissions granted to each of their groups.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
}

impl WritableUser {
    pub fn new(username: String, password: String) -> WritableUser {
        WritableUser {
            id: None,
            url: None,
            display: None,
            username,
            password,
            first_name: None,
            last_name: None,
            email: None,
            is_staff: None,
            is_active: None,
            date_joined: None,
            groups: None,
        }
    }
}
