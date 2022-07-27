/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSystemJumps200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseSystemJumps200Ok {
    /// ship_jumps integer
    #[serde(rename = "ship_jumps")]
    pub ship_jumps: i32,
    /// system_id integer
    #[serde(rename = "system_id")]
    pub system_id: i32,
}

impl GetUniverseSystemJumps200Ok {
    /// 200 ok object
    pub fn new(ship_jumps: i32, system_id: i32) -> GetUniverseSystemJumps200Ok {
        GetUniverseSystemJumps200Ok {
            ship_jumps,
            system_id,
        }
    }
}
