/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsLastWeekLastWeek : last_week object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsLastWeekLastWeek {
    /// Amount of kills
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
}

impl GetFwLeaderboardsLastWeekLastWeek {
    /// last_week object
    pub fn new() -> GetFwLeaderboardsLastWeekLastWeek {
        GetFwLeaderboardsLastWeekLastWeek {
            amount: None,
            faction_id: None,
        }
    }
}


