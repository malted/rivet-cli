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
pub struct GroupListSuggestedResponse {
	/// A list of group summaries.
	#[serde(rename = "groups")]
	pub groups: Vec<crate::models::GroupSummary>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl GroupListSuggestedResponse {
	pub fn new(
		groups: Vec<crate::models::GroupSummary>,
		watch: crate::models::WatchResponse,
	) -> GroupListSuggestedResponse {
		GroupListSuggestedResponse {
			groups,
			watch: Box::new(watch),
		}
	}
}
