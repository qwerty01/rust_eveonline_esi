/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdHead : head object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdHead {
    /// head_id integer
    #[serde(rename = "head_id")]
    pub head_id: i32,
    /// latitude number
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// longitude number
    #[serde(rename = "longitude")]
    pub longitude: f32,
}

impl GetCharactersCharacterIdPlanetsPlanetIdHead {
    /// head object
    pub fn new(
        head_id: i32,
        latitude: f32,
        longitude: f32,
    ) -> GetCharactersCharacterIdPlanetsPlanetIdHead {
        GetCharactersCharacterIdPlanetsPlanetIdHead {
            head_id,
            latitude,
            longitude,
        }
    }
}
