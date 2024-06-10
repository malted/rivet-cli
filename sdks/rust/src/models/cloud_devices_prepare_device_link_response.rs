/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudDevicesPrepareDeviceLinkResponse {
	#[serde(rename = "device_link_id")]
	pub device_link_id: uuid::Uuid,
	#[serde(rename = "device_link_token")]
	pub device_link_token: String,
	#[serde(rename = "device_link_url")]
	pub device_link_url: String,
}

impl CloudDevicesPrepareDeviceLinkResponse {
	pub fn new(
		device_link_id: uuid::Uuid,
		device_link_token: String,
		device_link_url: String,
	) -> CloudDevicesPrepareDeviceLinkResponse {
		CloudDevicesPrepareDeviceLinkResponse {
			device_link_id,
			device_link_token,
			device_link_url,
		}
	}
}
