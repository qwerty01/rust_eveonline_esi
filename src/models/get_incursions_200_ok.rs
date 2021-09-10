/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetIncursions200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIncursions200Ok {
    /// The constellation id in which this incursion takes place
    #[serde(rename = "constellation_id")]
    pub constellation_id: i32,
    /// The attacking faction's id
    #[serde(rename = "faction_id")]
    pub faction_id: i32,
    /// Whether the final encounter has boss or not
    #[serde(rename = "has_boss")]
    pub has_boss: bool,
    /// A list of infested solar system ids that are a part of this incursion
    #[serde(rename = "infested_solar_systems")]
    pub infested_solar_systems: Vec<i32>,
    /// Influence of this incursion as a float from 0 to 1
    #[serde(rename = "influence")]
    pub influence: f32,
    /// Staging solar system for this incursion
    #[serde(rename = "staging_solar_system_id")]
    pub staging_solar_system_id: i32,
    /// The state of this incursion
    #[serde(rename = "state")]
    pub state: State,
    /// The type of this incursion
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetIncursions200Ok {
    /// 200 ok object
    pub fn new(constellation_id: i32, faction_id: i32, has_boss: bool, infested_solar_systems: Vec<i32>, influence: f32, staging_solar_system_id: i32, state: State, _type: String) -> GetIncursions200Ok {
        GetIncursions200Ok {
            constellation_id,
            faction_id,
            has_boss,
            infested_solar_systems,
            influence,
            staging_solar_system_id,
            state,
            _type,
        }
    }
}

/// The state of this incursion
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "withdrawing")]
    Withdrawing,
    #[serde(rename = "mobilizing")]
    Mobilizing,
    #[serde(rename = "established")]
    Established,
}
