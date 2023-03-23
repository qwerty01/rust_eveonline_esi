/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetDogmaEffectsEffectIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDogmaEffectsEffectIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetDogmaEffectsEffectIdNotFound {
    /// Not found
    pub fn new() -> GetDogmaEffectsEffectIdNotFound {
        GetDogmaEffectsEffectIdNotFound { error: None }
    }
}
