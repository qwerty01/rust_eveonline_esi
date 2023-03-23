/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdAssetsNotFound : Requested page does not exist

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdAssetsNotFound {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetCharactersCharacterIdAssetsNotFound {
    /// Requested page does not exist
    pub fn new() -> GetCharactersCharacterIdAssetsNotFound {
        GetCharactersCharacterIdAssetsNotFound { error: None }
    }
}
