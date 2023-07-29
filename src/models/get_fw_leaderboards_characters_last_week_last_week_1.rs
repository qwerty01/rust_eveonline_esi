/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCharactersLastWeekLastWeek1 : last_week object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersLastWeekLastWeek1 {
    /// Amount of victory points
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// character_id integer
    #[serde(rename = "character_id", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i32>,
}

impl GetFwLeaderboardsCharactersLastWeekLastWeek1 {
    /// last_week object
    pub fn new() -> GetFwLeaderboardsCharactersLastWeekLastWeek1 {
        GetFwLeaderboardsCharactersLastWeekLastWeek1 {
            amount: None,
            character_id: None,
        }
    }
}


