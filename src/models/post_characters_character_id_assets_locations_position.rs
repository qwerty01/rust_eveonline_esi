/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostCharactersCharacterIdAssetsLocationsPosition : position object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdAssetsLocationsPosition {
    /// x number
    #[serde(rename = "x")]
    pub x: f64,
    /// y number
    #[serde(rename = "y")]
    pub y: f64,
    /// z number
    #[serde(rename = "z")]
    pub z: f64,
}

impl PostCharactersCharacterIdAssetsLocationsPosition {
    /// position object
    pub fn new(x: f64, y: f64, z: f64) -> PostCharactersCharacterIdAssetsLocationsPosition {
        PostCharactersCharacterIdAssetsLocationsPosition { x, y, z }
    }
}
