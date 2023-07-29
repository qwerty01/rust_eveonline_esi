/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsYesterdayYesterday : yesterday object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsYesterdayYesterday {
    /// Amount of kills
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
}

impl GetFwLeaderboardsCorporationsYesterdayYesterday {
    /// yesterday object
    pub fn new() -> GetFwLeaderboardsCorporationsYesterdayYesterday {
        GetFwLeaderboardsCorporationsYesterdayYesterday {
            amount: None,
            corporation_id: None,
        }
    }
}


