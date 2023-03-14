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
pub struct IdentitySearchOutput {
    /// The pagination anchor. 
    #[serde(rename = "anchor")]
    pub anchor: String,
    #[serde(rename = "identities")]
    pub identities: Vec<crate::models::IdentityHandle>,
}

impl IdentitySearchOutput {
    pub fn new(anchor: String, identities: Vec<crate::models::IdentityHandle>) -> IdentitySearchOutput {
        IdentitySearchOutput {
            anchor,
            identities,
        }
    }
}


