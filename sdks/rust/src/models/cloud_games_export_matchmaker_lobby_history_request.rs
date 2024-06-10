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
pub struct CloudGamesExportMatchmakerLobbyHistoryRequest {
	/// Unsigned 64 bit integer.
	#[serde(rename = "query_end")]
	pub query_end: i64,
	/// Unsigned 64 bit integer.
	#[serde(rename = "query_start")]
	pub query_start: i64,
}

impl CloudGamesExportMatchmakerLobbyHistoryRequest {
	pub fn new(query_end: i64, query_start: i64) -> CloudGamesExportMatchmakerLobbyHistoryRequest {
		CloudGamesExportMatchmakerLobbyHistoryRequest {
			query_end,
			query_start,
		}
	}
}
