/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseConstellationsConstellationIdPosition : position object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseConstellationsConstellationIdPosition {
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

impl GetUniverseConstellationsConstellationIdPosition {
    /// position object
    pub fn new(x: f64, y: f64, z: f64) -> GetUniverseConstellationsConstellationIdPosition {
        GetUniverseConstellationsConstellationIdPosition { x, y, z }
    }
}
