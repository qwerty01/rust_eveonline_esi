/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutFleetsFleetIdMembersMemberIdMovement : movement object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFleetsFleetIdMembersMemberIdMovement {
    /// If a character is moved to the `fleet_commander` role, neither `wing_id` or `squad_id` should be specified. If a character is moved to the `wing_commander` role, only `wing_id` should be specified. If a character is moved to the `squad_commander` role, both `wing_id` and `squad_id` should be specified. If a character is moved to the `squad_member` role, both `wing_id` and `squad_id` should be specified.
    #[serde(rename = "role")]
    pub role: Role,
    /// squad_id integer
    #[serde(rename = "squad_id", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<i64>,
    /// wing_id integer
    #[serde(rename = "wing_id", skip_serializing_if = "Option::is_none")]
    pub wing_id: Option<i64>,
}

impl PutFleetsFleetIdMembersMemberIdMovement {
    /// movement object
    pub fn new(role: Role) -> PutFleetsFleetIdMembersMemberIdMovement {
        PutFleetsFleetIdMembersMemberIdMovement {
            role,
            squad_id: None,
            wing_id: None,
        }
    }
}

/// If a character is moved to the `fleet_commander` role, neither `wing_id` or `squad_id` should be specified. If a character is moved to the `wing_commander` role, only `wing_id` should be specified. If a character is moved to the `squad_commander` role, both `wing_id` and `squad_id` should be specified. If a character is moved to the `squad_member` role, both `wing_id` and `squad_id` should be specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "fleet_commander")]
    FleetCommander,
    #[serde(rename = "wing_commander")]
    WingCommander,
    #[serde(rename = "squad_commander")]
    SquadCommander,
    #[serde(rename = "squad_member")]
    SquadMember,
}

impl Default for Role {
    fn default() -> Role {
        Self::FleetCommander
    }
}

