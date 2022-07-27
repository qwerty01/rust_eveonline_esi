/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFwSystems200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwSystems200Ok {
    /// contested string
    #[serde(rename = "contested")]
    pub contested: Contested,
    /// occupier_faction_id integer
    #[serde(rename = "occupier_faction_id")]
    pub occupier_faction_id: i32,
    /// owner_faction_id integer
    #[serde(rename = "owner_faction_id")]
    pub owner_faction_id: i32,
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// victory_points integer
    #[serde(rename = "victory_points")]
    pub victory_points: i32,
    /// victory_points_threshold integer
    #[serde(rename = "victory_points_threshold")]
    pub victory_points_threshold: i32,
}

impl GetFwSystems200Ok {
    /// 200 ok object
    pub fn new(
        contested: Contested,
        occupier_faction_id: i32,
        owner_faction_id: i32,
        solar_system_id: i32,
        victory_points: i32,
        victory_points_threshold: i32,
    ) -> GetFwSystems200Ok {
        GetFwSystems200Ok {
            contested,
            occupier_faction_id,
            owner_faction_id,
            solar_system_id,
            victory_points,
            victory_points_threshold,
        }
    }
}

/// contested string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Contested {
    #[serde(rename = "captured")]
    Captured,
    #[serde(rename = "contested")]
    Contested,
    #[serde(rename = "uncontested")]
    Uncontested,
    #[serde(rename = "vulnerable")]
    Vulnerable,
}

impl Default for Contested {
    fn default() -> Contested {
        Self::Captured
    }
}
