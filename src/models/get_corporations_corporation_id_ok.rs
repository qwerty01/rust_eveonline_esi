/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdOk {
    /// ID of the alliance that corporation is a member of, if any
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// ceo_id integer
    #[serde(rename = "ceo_id")]
    pub ceo_id: i32,
    /// creator_id integer
    #[serde(rename = "creator_id")]
    pub creator_id: i32,
    /// date_founded string
    #[serde(rename = "date_founded", skip_serializing_if = "Option::is_none")]
    pub date_founded: Option<String>,
    /// description string
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// home_station_id integer
    #[serde(rename = "home_station_id", skip_serializing_if = "Option::is_none")]
    pub home_station_id: Option<i32>,
    /// member_count integer
    #[serde(rename = "member_count")]
    pub member_count: i32,
    /// the full name of the corporation
    #[serde(rename = "name")]
    pub name: String,
    /// shares integer
    #[serde(rename = "shares", skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    /// tax_rate number
    #[serde(rename = "tax_rate")]
    pub tax_rate: f32,
    /// the short name of the corporation
    #[serde(rename = "ticker")]
    pub ticker: String,
    /// url string
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// war_eligible boolean
    #[serde(rename = "war_eligible", skip_serializing_if = "Option::is_none")]
    pub war_eligible: Option<bool>,
}

impl GetCorporationsCorporationIdOk {
    /// 200 ok object
    pub fn new(ceo_id: i32, creator_id: i32, member_count: i32, name: String, tax_rate: f32, ticker: String) -> GetCorporationsCorporationIdOk {
        GetCorporationsCorporationIdOk {
            alliance_id: None,
            ceo_id,
            creator_id,
            date_founded: None,
            description: None,
            faction_id: None,
            home_station_id: None,
            member_count,
            name,
            shares: None,
            tax_rate,
            ticker,
            url: None,
            war_eligible: None,
        }
    }
}


