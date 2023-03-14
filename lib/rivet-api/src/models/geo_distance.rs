/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GeoDistance : Distance available in multiple units.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeoDistance {
    #[serde(rename = "kilometers")]
    pub kilometers: f64,
    #[serde(rename = "miles")]
    pub miles: f64,
}

impl GeoDistance {
    /// Distance available in multiple units.
    pub fn new(kilometers: f64, miles: f64) -> GeoDistance {
        GeoDistance {
            kilometers,
            miles,
        }
    }
}


