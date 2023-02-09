/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudDevicesCompleteDeviceLinkInput {
    /// Documentation at https://jwt.io/
    #[serde(rename = "device_link_token")]
    pub device_link_token: String,
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
}

impl CloudDevicesCompleteDeviceLinkInput {
    pub fn new(device_link_token: String, game_id: uuid::Uuid) -> CloudDevicesCompleteDeviceLinkInput {
        CloudDevicesCompleteDeviceLinkInput {
            device_link_token,
            game_id,
        }
    }
}


