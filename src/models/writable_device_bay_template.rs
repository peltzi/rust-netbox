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
pub struct WritableDeviceBayTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: i32,
    #[serde(rename = "name")]
    pub name: String,
}

impl WritableDeviceBayTemplate {
    pub fn new(device_type: i32, name: String) -> WritableDeviceBayTemplate {
        WritableDeviceBayTemplate {
            id: None,
            device_type,
            name,
        }
    }
}

