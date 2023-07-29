/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsLastWeekLastWeek : last_week object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsLastWeekLastWeek {
    /// Amount of kills
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
}

impl GetFwLeaderboardsCorporationsLastWeekLastWeek {
    /// last_week object
    pub fn new() -> GetFwLeaderboardsCorporationsLastWeekLastWeek {
        GetFwLeaderboardsCorporationsLastWeekLastWeek {
            amount: None,
            corporation_id: None,
        }
    }
}


