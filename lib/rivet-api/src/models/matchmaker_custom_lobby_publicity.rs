/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MatchmakerCustomLobbyPublicity {
	#[serde(rename = "public")]
	Public,
	#[serde(rename = "private")]
	Private,
}

impl ToString for MatchmakerCustomLobbyPublicity {
	fn to_string(&self) -> String {
		match self {
			Self::Public => String::from("public"),
			Self::Private => String::from("private"),
		}
	}
}

impl Default for MatchmakerCustomLobbyPublicity {
	fn default() -> MatchmakerCustomLobbyPublicity {
		Self::Public
	}
}
