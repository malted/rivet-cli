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
pub struct GlobalEventChatThreadRemove {
    #[serde(rename = "thread_id")]
    pub thread_id: uuid::Uuid,
}

impl GlobalEventChatThreadRemove {
    pub fn new(thread_id: uuid::Uuid) -> GlobalEventChatThreadRemove {
        GlobalEventChatThreadRemove {
            thread_id,
        }
    }
}


