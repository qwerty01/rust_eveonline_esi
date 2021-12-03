/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails : factory_details object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails {
    /// schematic_id integer
    #[serde(rename = "schematic_id")]
    pub schematic_id: i32,
}

impl GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails {
    /// factory_details object
    pub fn new(schematic_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails {
        GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails { schematic_id }
    }
}
