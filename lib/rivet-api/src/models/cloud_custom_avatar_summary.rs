/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudCustomAvatarSummary : A custom avatar summary.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudCustomAvatarSummary {
	/// Whether or not this custom avatar has completely been uploaded.
	#[serde(rename = "complete")]
	pub complete: bool,
	/// Unsigned 64 bit integer.
	#[serde(rename = "content_length")]
	pub content_length: i64,
	/// RFC3339 timestamp.
	#[serde(rename = "create_ts")]
	pub create_ts: String,
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	#[serde(rename = "upload_id")]
	pub upload_id: uuid::Uuid,
	/// The URL of this custom avatar image. Only present if upload is complete.
	#[serde(rename = "url", skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

impl CloudCustomAvatarSummary {
	/// A custom avatar summary.
	pub fn new(
		complete: bool,
		content_length: i64,
		create_ts: String,
		display_name: String,
		upload_id: uuid::Uuid,
	) -> CloudCustomAvatarSummary {
		CloudCustomAvatarSummary {
			complete,
			content_length,
			create_ts,
			display_name,
			upload_id,
			url: None,
		}
	}
}
