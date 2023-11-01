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
pub struct IdentityGetGameLinkResponse {
	#[serde(rename = "current_identity")]
	pub current_identity: Box<crate::models::IdentityHandle>,
	#[serde(rename = "game")]
	pub game: Box<crate::models::GameHandle>,
	#[serde(rename = "new_identity", skip_serializing_if = "Option::is_none")]
	pub new_identity: Option<Box<crate::models::IdentityGetGameLinkNewIdentity>>,
	#[serde(rename = "status")]
	pub status: crate::models::IdentityGameLinkStatus,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl IdentityGetGameLinkResponse {
	pub fn new(
		current_identity: crate::models::IdentityHandle,
		game: crate::models::GameHandle,
		status: crate::models::IdentityGameLinkStatus,
		watch: crate::models::WatchResponse,
	) -> IdentityGetGameLinkResponse {
		IdentityGetGameLinkResponse {
			current_identity: Box::new(current_identity),
			game: Box::new(game),
			new_identity: None,
			status,
			watch: Box::new(watch),
		}
	}
}
