/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetSovereigntyMap200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetSovereigntyMap200Ok {
    /// alliance_id integer
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// system_id integer
    #[serde(rename = "system_id")]
    pub system_id: i32,
}

impl GetSovereigntyMap200Ok {
    /// 200 ok object
    pub fn new(system_id: i32) -> GetSovereigntyMap200Ok {
        GetSovereigntyMap200Ok {
            alliance_id: None,
            corporation_id: None,
            faction_id: None,
            system_id,
        }
    }
}
