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
pub struct InlineResponse2008 {
    #[serde(rename = "sites")]
    pub sites: Vec<crate::models::CdnSiteSummary>,
}

impl InlineResponse2008 {
    pub fn new(sites: Vec<crate::models::CdnSiteSummary>) -> InlineResponse2008 {
        InlineResponse2008 {
            sites,
        }
    }
}

