/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSystemsSystemIdPosition : position object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseSystemsSystemIdPosition {
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

impl GetUniverseSystemsSystemIdPosition {
    /// position object
    pub fn new(x: f64, y: f64, z: f64) -> GetUniverseSystemsSystemIdPosition {
        GetUniverseSystemsSystemIdPosition {
            x,
            y,
            z,
        }
    }
}


