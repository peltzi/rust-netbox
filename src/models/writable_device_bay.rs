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
pub struct WritableDeviceBay {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "installed_device", skip_serializing_if = "Option::is_none")]
    pub installed_device: Option<i32>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WritableDeviceBay {
    pub fn new(device: i32, name: String) -> WritableDeviceBay {
        WritableDeviceBay {
            id: None,
            device,
            name,
            description: None,
            installed_device: None,
            tags: None,
        }
    }
}


