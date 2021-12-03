/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniversePlanetsPlanetIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniversePlanetsPlanetIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniversePlanetsPlanetIdNotFound {
    /// Not found
    pub fn new() -> GetUniversePlanetsPlanetIdNotFound {
        GetUniversePlanetsPlanetIdNotFound { error: None }
    }
}
