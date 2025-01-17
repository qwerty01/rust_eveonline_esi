/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsOk {
    #[serde(rename = "kills")]
    pub kills: Box<crate::models::GetFwLeaderboardsCorporationsKills>,
    #[serde(rename = "victory_points")]
    pub victory_points: Box<crate::models::GetFwLeaderboardsCorporationsVictoryPoints>,
}

impl GetFwLeaderboardsCorporationsOk {
    /// 200 ok object
    pub fn new(kills: crate::models::GetFwLeaderboardsCorporationsKills, victory_points: crate::models::GetFwLeaderboardsCorporationsVictoryPoints) -> GetFwLeaderboardsCorporationsOk {
        GetFwLeaderboardsCorporationsOk {
            kills: Box::new(kills),
            victory_points: Box::new(victory_points),
        }
    }
}


