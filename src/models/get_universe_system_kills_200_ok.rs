/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSystemKills200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseSystemKills200Ok {
    /// Number of NPC ships killed in this system
    #[serde(rename = "npc_kills")]
    pub npc_kills: i32,
    /// Number of pods killed in this system
    #[serde(rename = "pod_kills")]
    pub pod_kills: i32,
    /// Number of player ships killed in this system
    #[serde(rename = "ship_kills")]
    pub ship_kills: i32,
    /// system_id integer
    #[serde(rename = "system_id")]
    pub system_id: i32,
}

impl GetUniverseSystemKills200Ok {
    /// 200 ok object
    pub fn new(
        npc_kills: i32,
        pod_kills: i32,
        ship_kills: i32,
        system_id: i32,
    ) -> GetUniverseSystemKills200Ok {
        GetUniverseSystemKills200Ok {
            npc_kills,
            pod_kills,
            ship_kills,
            system_id,
        }
    }
}
