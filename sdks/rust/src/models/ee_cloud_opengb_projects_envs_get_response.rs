/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EeCloudOpengbProjectsEnvsGetResponse {
	#[serde(rename = "environment")]
	pub environment: Box<crate::models::EeOpengbEnvironment>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl EeCloudOpengbProjectsEnvsGetResponse {
	pub fn new(
		environment: crate::models::EeOpengbEnvironment,
		watch: crate::models::WatchResponse,
	) -> EeCloudOpengbProjectsEnvsGetResponse {
		EeCloudOpengbProjectsEnvsGetResponse {
			environment: Box::new(environment),
			watch: Box::new(watch),
		}
	}
}