/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetAlliancesAllianceIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAlliancesAllianceIdOk {
    /// ID of the corporation that created the alliance
    #[serde(rename = "creator_corporation_id")]
    pub creator_corporation_id: i32,
    /// ID of the character that created the alliance
    #[serde(rename = "creator_id")]
    pub creator_id: i32,
    /// date_founded string
    #[serde(rename = "date_founded")]
    pub date_founded: String,
    /// the executor corporation ID, if this alliance is not closed
    #[serde(
        rename = "executor_corporation_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub executor_corporation_id: Option<i32>,
    /// Faction ID this alliance is fighting for, if this alliance is enlisted in factional warfare
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// the full name of the alliance
    #[serde(rename = "name")]
    pub name: String,
    /// the short name of the alliance
    #[serde(rename = "ticker")]
    pub ticker: String,
}

impl GetAlliancesAllianceIdOk {
    /// 200 ok object
    pub fn new(
        creator_corporation_id: i32,
        creator_id: i32,
        date_founded: String,
        name: String,
        ticker: String,
    ) -> GetAlliancesAllianceIdOk {
        GetAlliancesAllianceIdOk {
            creator_corporation_id,
            creator_id,
            date_founded,
            executor_corporation_id: None,
            faction_id: None,
            name,
            ticker,
        }
    }
}
