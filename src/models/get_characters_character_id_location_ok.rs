/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdLocationOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdLocationOk {
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// station_id integer
    #[serde(rename = "station_id", skip_serializing_if = "Option::is_none")]
    pub station_id: Option<i32>,
    /// structure_id integer
    #[serde(rename = "structure_id", skip_serializing_if = "Option::is_none")]
    pub structure_id: Option<i64>,
}

impl GetCharactersCharacterIdLocationOk {
    /// 200 ok object
    pub fn new(solar_system_id: i32) -> GetCharactersCharacterIdLocationOk {
        GetCharactersCharacterIdLocationOk {
            solar_system_id,
            station_id: None,
            structure_id: None,
        }
    }
}


