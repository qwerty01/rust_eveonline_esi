/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdRoute : route object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdRoute {
    /// content_type_id integer
    #[serde(rename = "content_type_id")]
    pub content_type_id: i32,
    /// destination_pin_id integer
    #[serde(rename = "destination_pin_id")]
    pub destination_pin_id: i64,
    /// quantity number
    #[serde(rename = "quantity")]
    pub quantity: f32,
    /// route_id integer
    #[serde(rename = "route_id")]
    pub route_id: i64,
    /// source_pin_id integer
    #[serde(rename = "source_pin_id")]
    pub source_pin_id: i64,
    /// list of pin ID waypoints
    #[serde(rename = "waypoints", skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<Vec<i64>>,
}

impl GetCharactersCharacterIdPlanetsPlanetIdRoute {
    /// route object
    pub fn new(
        content_type_id: i32,
        destination_pin_id: i64,
        quantity: f32,
        route_id: i64,
        source_pin_id: i64,
    ) -> GetCharactersCharacterIdPlanetsPlanetIdRoute {
        GetCharactersCharacterIdPlanetsPlanetIdRoute {
            content_type_id,
            destination_pin_id,
            quantity,
            route_id,
            source_pin_id,
            waypoints: None,
        }
    }
}
