/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionFullAllOf {
    #[serde(rename = "config")]
    pub config: Box<crate::models::CloudVersionConfig>,
}

impl VersionFullAllOf {
    pub fn new(config: crate::models::CloudVersionConfig) -> VersionFullAllOf {
        VersionFullAllOf {
            config: Box::new(config),
        }
    }
}

