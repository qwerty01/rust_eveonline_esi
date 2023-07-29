/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwWars200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwWars200Ok {
    /// The faction ID of the enemy faction.
    #[serde(rename = "against_id")]
    pub against_id: i32,
    /// faction_id integer
    #[serde(rename = "faction_id")]
    pub faction_id: i32,
}

impl GetFwWars200Ok {
    /// 200 ok object
    pub fn new(against_id: i32, faction_id: i32) -> GetFwWars200Ok {
        GetFwWars200Ok {
            against_id,
            faction_id,
        }
    }
}


