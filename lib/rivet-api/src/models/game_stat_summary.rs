/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GameStatSummary : A game statistic summary.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GameStatSummary {
	#[serde(rename = "game")]
	pub game: Box<crate::models::GameHandle>,
	#[serde(rename = "stats")]
	pub stats: Vec<crate::models::GameStat>,
}

impl GameStatSummary {
	/// A game statistic summary.
	pub fn new(
		game: crate::models::GameHandle,
		stats: Vec<crate::models::GameStat>,
	) -> GameStatSummary {
		GameStatSummary {
			game: Box::new(game),
			stats,
		}
	}
}
