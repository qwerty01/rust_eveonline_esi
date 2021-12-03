/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetKillmailsKillmailIdKillmailHashPosition : Coordinates of the victim in Cartesian space relative to the Sun

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashPosition {
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

impl GetKillmailsKillmailIdKillmailHashPosition {
    /// Coordinates of the victim in Cartesian space relative to the Sun
    pub fn new(x: f64, y: f64, z: f64) -> GetKillmailsKillmailIdKillmailHashPosition {
        GetKillmailsKillmailIdKillmailHashPosition { x, y, z }
    }
}
