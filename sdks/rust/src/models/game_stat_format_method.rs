/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GameStatFormatMethod : A value denoting the format method of a game statistic.

/// A value denoting the format method of a game statistic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GameStatFormatMethod {
	#[serde(rename = "integer")]
	Integer,
	#[serde(rename = "float_1")]
	Float1,
	#[serde(rename = "float_2")]
	Float2,
	#[serde(rename = "float_3")]
	Float3,
	#[serde(rename = "duration_minute")]
	DurationMinute,
	#[serde(rename = "duration_second")]
	DurationSecond,
	#[serde(rename = "duration_hundredth_second")]
	DurationHundredthSecond,
}

impl ToString for GameStatFormatMethod {
	fn to_string(&self) -> String {
		match self {
			Self::Integer => String::from("integer"),
			Self::Float1 => String::from("float_1"),
			Self::Float2 => String::from("float_2"),
			Self::Float3 => String::from("float_3"),
			Self::DurationMinute => String::from("duration_minute"),
			Self::DurationSecond => String::from("duration_second"),
			Self::DurationHundredthSecond => String::from("duration_hundredth_second"),
		}
	}
}

impl Default for GameStatFormatMethod {
	fn default() -> GameStatFormatMethod {
		Self::Integer
	}
}