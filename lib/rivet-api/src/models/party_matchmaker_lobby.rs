/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartyMatchmakerLobby : A party lobby.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartyMatchmakerLobby {
    #[serde(rename = "lobby_id")]
    pub lobby_id: uuid::Uuid,
}

impl PartyMatchmakerLobby {
    /// A party lobby.
    pub fn new(lobby_id: uuid::Uuid) -> PartyMatchmakerLobby {
        PartyMatchmakerLobby {
            lobby_id,
        }
    }
}

