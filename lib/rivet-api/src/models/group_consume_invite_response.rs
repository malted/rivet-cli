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
pub struct GroupConsumeInviteResponse {
	#[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
	pub group_id: Option<uuid::Uuid>,
}

impl GroupConsumeInviteResponse {
	pub fn new() -> GroupConsumeInviteResponse {
		GroupConsumeInviteResponse { group_id: None }
	}
}
