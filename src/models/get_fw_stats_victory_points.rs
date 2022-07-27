/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFwStatsVictoryPoints : Summary of victory points gained for the given faction

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwStatsVictoryPoints {
    /// Last week's victory points gained
    #[serde(rename = "last_week")]
    pub last_week: i32,
    /// Total victory points gained since faction warfare began
    #[serde(rename = "total")]
    pub total: i32,
    /// Yesterday's victory points gained
    #[serde(rename = "yesterday")]
    pub yesterday: i32,
}

impl GetFwStatsVictoryPoints {
    /// Summary of victory points gained for the given faction
    pub fn new(last_week: i32, total: i32, yesterday: i32) -> GetFwStatsVictoryPoints {
        GetFwStatsVictoryPoints {
            last_week,
            total,
            yesterday,
        }
    }
}
