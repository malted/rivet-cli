/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChatSendTopic : Topic to send a chat message to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSendTopic {
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "identity_id", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<uuid::Uuid>,
    #[serde(rename = "party_id", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<uuid::Uuid>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<uuid::Uuid>,
}

impl ChatSendTopic {
    /// Topic to send a chat message to.
    pub fn new() -> ChatSendTopic {
        ChatSendTopic {
            group_id: None,
            identity_id: None,
            party_id: None,
            thread_id: None,
        }
    }
}


