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
pub struct CloudGamesCreateGameVersionOutput {
    /// A universally unique identifier.
    #[serde(rename = "version_id")]
    pub version_id: String,
}

impl CloudGamesCreateGameVersionOutput {
    pub fn new(version_id: String) -> CloudGamesCreateGameVersionOutput {
        CloudGamesCreateGameVersionOutput {
            version_id,
        }
    }
}


