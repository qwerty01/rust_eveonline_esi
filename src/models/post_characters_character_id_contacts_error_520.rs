/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostCharactersCharacterIdContactsError520 : Error 520

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdContactsError520 {
    /// Error 520 message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PostCharactersCharacterIdContactsError520 {
    /// Error 520
    pub fn new() -> PostCharactersCharacterIdContactsError520 {
        PostCharactersCharacterIdContactsError520 { error: None }
    }
}
