/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsYesterdayYesterday1 : yesterday object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsYesterdayYesterday1 {
    /// Amount of victory points
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
}

impl GetFwLeaderboardsCorporationsYesterdayYesterday1 {
    /// yesterday object
    pub fn new() -> GetFwLeaderboardsCorporationsYesterdayYesterday1 {
        GetFwLeaderboardsCorporationsYesterdayYesterday1 {
            amount: None,
            corporation_id: None,
        }
    }
}
