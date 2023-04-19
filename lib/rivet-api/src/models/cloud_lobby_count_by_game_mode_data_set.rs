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
#[serde(deny_unknown_fields)]
pub struct CloudLobbyCountByGameModeDataSet {
	#[serde(rename = "game_mode_name_id")]
	pub game_mode_name_id: Vec<String>,
	#[serde(rename = "lobby_count")]
	pub lobby_count: Vec<i64>,
	#[serde(rename = "ts")]
	pub ts: Vec<i64>,
}

impl CloudLobbyCountByGameModeDataSet {
	pub fn new(
		game_mode_name_id: Vec<String>,
		lobby_count: Vec<i64>,
		ts: Vec<i64>,
	) -> CloudLobbyCountByGameModeDataSet {
		CloudLobbyCountByGameModeDataSet {
			game_mode_name_id,
			lobby_count,
			ts,
		}
	}
}