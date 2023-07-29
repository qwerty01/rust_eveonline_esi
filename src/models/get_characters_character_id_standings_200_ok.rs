/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdStandings200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStandings200Ok {
    /// from_id integer
    #[serde(rename = "from_id")]
    pub from_id: i32,
    /// from_type string
    #[serde(rename = "from_type")]
    pub from_type: FromType,
    /// standing number
    #[serde(rename = "standing")]
    pub standing: f32,
}

impl GetCharactersCharacterIdStandings200Ok {
    /// 200 ok object
    pub fn new(from_id: i32, from_type: FromType, standing: f32) -> GetCharactersCharacterIdStandings200Ok {
        GetCharactersCharacterIdStandings200Ok {
            from_id,
            from_type,
            standing,
        }
    }
}

/// from_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FromType {
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "npc_corp")]
    NpcCorp,
    #[serde(rename = "faction")]
    Faction,
}

impl Default for FromType {
    fn default() -> FromType {
        Self::Agent
    }
}

