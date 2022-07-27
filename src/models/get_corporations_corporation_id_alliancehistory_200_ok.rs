/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdAlliancehistory200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdAlliancehistory200Ok {
    /// alliance_id integer
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// True if the alliance has been closed
    #[serde(rename = "is_deleted", skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// An incrementing ID that can be used to canonically establish order of records in cases where dates may be ambiguous
    #[serde(rename = "record_id")]
    pub record_id: i32,
    /// start_date string
    #[serde(rename = "start_date")]
    pub start_date: String,
}

impl GetCorporationsCorporationIdAlliancehistory200Ok {
    /// 200 ok object
    pub fn new(
        record_id: i32,
        start_date: String,
    ) -> GetCorporationsCorporationIdAlliancehistory200Ok {
        GetCorporationsCorporationIdAlliancehistory200Ok {
            alliance_id: None,
            is_deleted: None,
            record_id,
            start_date,
        }
    }
}
