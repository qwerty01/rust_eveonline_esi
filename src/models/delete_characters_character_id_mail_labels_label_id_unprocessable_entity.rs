/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteCharactersCharacterIdMailLabelsLabelIdUnprocessableEntity : Unprocessable entity

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCharactersCharacterIdMailLabelsLabelIdUnprocessableEntity {
    /// Unprocessable entity message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl DeleteCharactersCharacterIdMailLabelsLabelIdUnprocessableEntity {
    /// Unprocessable entity
    pub fn new() -> DeleteCharactersCharacterIdMailLabelsLabelIdUnprocessableEntity {
        DeleteCharactersCharacterIdMailLabelsLabelIdUnprocessableEntity { error: None }
    }
}
