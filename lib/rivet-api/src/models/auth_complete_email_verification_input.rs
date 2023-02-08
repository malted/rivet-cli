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
pub struct AuthCompleteEmailVerificationInput {
    /// The code sent to the requestee's email.
    #[serde(rename = "code")]
    pub code: String,
    /// A universally unique identifier.
    #[serde(rename = "verification_id")]
    pub verification_id: String,
}

impl AuthCompleteEmailVerificationInput {
    pub fn new(code: String, verification_id: String) -> AuthCompleteEmailVerificationInput {
        AuthCompleteEmailVerificationInput {
            code,
            verification_id,
        }
    }
}

