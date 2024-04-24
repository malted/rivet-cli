/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig : **Deprecated: use GameMode instead** Configuration for how many idle lobbies a game version should have.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig {
	/// Unsigned 32 bit integer.
	#[serde(rename = "max_idle_lobbies")]
	pub max_idle_lobbies: i32,
	/// Unsigned 32 bit integer.
	#[serde(rename = "min_idle_lobbies")]
	pub min_idle_lobbies: i32,
}

impl CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig {
	/// **Deprecated: use GameMode instead** Configuration for how many idle lobbies a game version should have.
	pub fn new(
		max_idle_lobbies: i32,
		min_idle_lobbies: i32,
	) -> CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig {
		CloudVersionMatchmakerLobbyGroupIdleLobbiesConfig {
			max_idle_lobbies,
			min_idle_lobbies,
		}
	}
}