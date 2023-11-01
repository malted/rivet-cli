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
pub struct KvGetBatchResponse {
	#[serde(rename = "entries")]
	pub entries: Vec<crate::models::KvEntry>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl KvGetBatchResponse {
	pub fn new(
		entries: Vec<crate::models::KvEntry>,
		watch: crate::models::WatchResponse,
	) -> KvGetBatchResponse {
		KvGetBatchResponse {
			entries,
			watch: Box::new(watch),
		}
	}
}
