/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdFleetOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFleetOk {
    /// The character's current fleet ID
    #[serde(rename = "fleet_id")]
    pub fleet_id: i64,
    /// Member’s role in fleet
    #[serde(rename = "role")]
    pub role: Role,
    /// ID of the squad the member is in. If not applicable, will be set to -1
    #[serde(rename = "squad_id")]
    pub squad_id: i64,
    /// ID of the wing the member is in. If not applicable, will be set to -1
    #[serde(rename = "wing_id")]
    pub wing_id: i64,
}

impl GetCharactersCharacterIdFleetOk {
    /// 200 ok object
    pub fn new(
        fleet_id: i64,
        role: Role,
        squad_id: i64,
        wing_id: i64,
    ) -> GetCharactersCharacterIdFleetOk {
        GetCharactersCharacterIdFleetOk {
            fleet_id,
            role,
            squad_id,
            wing_id,
        }
    }
}

/// Member’s role in fleet
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "fleet_commander")]
    FleetCommander,
    #[serde(rename = "squad_commander")]
    SquadCommander,
    #[serde(rename = "squad_member")]
    SquadMember,
    #[serde(rename = "wing_commander")]
    WingCommander,
}

impl Default for Role {
    fn default() -> Role {
        Self::FleetCommander
    }
}
