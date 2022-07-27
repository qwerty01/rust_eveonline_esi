/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsKills : Top 4 rankings of factions by number of kills from yesterday, last week and in total

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsKills {
    /// Top 4 ranking of factions active in faction warfare by total kills. A faction is considered \"active\" if they have participated in faction warfare in the past 14 days
    #[serde(rename = "active_total")]
    pub active_total: Vec<crate::models::GetFwLeaderboardsActiveTotalActiveTotal>,
    /// Top 4 ranking of factions by kills in the past week
    #[serde(rename = "last_week")]
    pub last_week: Vec<crate::models::GetFwLeaderboardsLastWeekLastWeek>,
    /// Top 4 ranking of factions by kills in the past day
    #[serde(rename = "yesterday")]
    pub yesterday: Vec<crate::models::GetFwLeaderboardsYesterdayYesterday>,
}

impl GetFwLeaderboardsKills {
    /// Top 4 rankings of factions by number of kills from yesterday, last week and in total
    pub fn new(
        active_total: Vec<crate::models::GetFwLeaderboardsActiveTotalActiveTotal>,
        last_week: Vec<crate::models::GetFwLeaderboardsLastWeekLastWeek>,
        yesterday: Vec<crate::models::GetFwLeaderboardsYesterdayYesterday>,
    ) -> GetFwLeaderboardsKills {
        GetFwLeaderboardsKills {
            active_total,
            last_week,
            yesterday,
        }
    }
}
