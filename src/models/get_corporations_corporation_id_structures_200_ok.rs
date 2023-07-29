/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdStructures200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStructures200Ok {
    /// ID of the corporation that owns the structure
    #[serde(rename = "corporation_id")]
    pub corporation_id: i32,
    /// Date on which the structure will run out of fuel
    #[serde(rename = "fuel_expires", skip_serializing_if = "Option::is_none")]
    pub fuel_expires: Option<String>,
    /// The structure name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The date and time when the structure's newly requested reinforcement times (e.g. next_reinforce_hour and next_reinforce_day) will take effect
    #[serde(rename = "next_reinforce_apply", skip_serializing_if = "Option::is_none")]
    pub next_reinforce_apply: Option<String>,
    /// The requested change to reinforce_hour that will take effect at the time shown by next_reinforce_apply
    #[serde(rename = "next_reinforce_hour", skip_serializing_if = "Option::is_none")]
    pub next_reinforce_hour: Option<i32>,
    /// The id of the ACL profile for this citadel
    #[serde(rename = "profile_id")]
    pub profile_id: i32,
    /// The hour of day that determines the four hour window when the structure will randomly exit its reinforcement periods and become vulnerable to attack against its armor and/or hull. The structure will become vulnerable at a random time that is +/- 2 hours centered on the value of this property
    #[serde(rename = "reinforce_hour", skip_serializing_if = "Option::is_none")]
    pub reinforce_hour: Option<i32>,
    /// Contains a list of service upgrades, and their state
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::GetCorporationsCorporationIdStructuresService>>,
    /// state string
    #[serde(rename = "state")]
    pub state: State,
    /// Date at which the structure will move to it's next state
    #[serde(rename = "state_timer_end", skip_serializing_if = "Option::is_none")]
    pub state_timer_end: Option<String>,
    /// Date at which the structure entered it's current state
    #[serde(rename = "state_timer_start", skip_serializing_if = "Option::is_none")]
    pub state_timer_start: Option<String>,
    /// The Item ID of the structure
    #[serde(rename = "structure_id")]
    pub structure_id: i64,
    /// The solar system the structure is in
    #[serde(rename = "system_id")]
    pub system_id: i32,
    /// The type id of the structure
    #[serde(rename = "type_id")]
    pub type_id: i32,
    /// Date at which the structure will unanchor
    #[serde(rename = "unanchors_at", skip_serializing_if = "Option::is_none")]
    pub unanchors_at: Option<String>,
}

impl GetCorporationsCorporationIdStructures200Ok {
    /// 200 ok object
    pub fn new(corporation_id: i32, profile_id: i32, state: State, structure_id: i64, system_id: i32, type_id: i32) -> GetCorporationsCorporationIdStructures200Ok {
        GetCorporationsCorporationIdStructures200Ok {
            corporation_id,
            fuel_expires: None,
            name: None,
            next_reinforce_apply: None,
            next_reinforce_hour: None,
            profile_id,
            reinforce_hour: None,
            services: None,
            state,
            state_timer_end: None,
            state_timer_start: None,
            structure_id,
            system_id,
            type_id,
            unanchors_at: None,
        }
    }
}

/// state string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "anchor_vulnerable")]
    AnchorVulnerable,
    #[serde(rename = "anchoring")]
    Anchoring,
    #[serde(rename = "armor_reinforce")]
    ArmorReinforce,
    #[serde(rename = "armor_vulnerable")]
    ArmorVulnerable,
    #[serde(rename = "deploy_vulnerable")]
    DeployVulnerable,
    #[serde(rename = "fitting_invulnerable")]
    FittingInvulnerable,
    #[serde(rename = "hull_reinforce")]
    HullReinforce,
    #[serde(rename = "hull_vulnerable")]
    HullVulnerable,
    #[serde(rename = "online_deprecated")]
    OnlineDeprecated,
    #[serde(rename = "onlining_vulnerable")]
    OnliningVulnerable,
    #[serde(rename = "shield_vulnerable")]
    ShieldVulnerable,
    #[serde(rename = "unanchored")]
    Unanchored,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for State {
    fn default() -> State {
        Self::AnchorVulnerable
    }
}

