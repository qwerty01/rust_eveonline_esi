/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdCorporationhistory200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdCorporationhistory200Ok {
    /// corporation_id integer
    #[serde(rename = "corporation_id")]
    pub corporation_id: i32,
    /// True if the corporation has been deleted
    #[serde(rename = "is_deleted", skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// An incrementing ID that can be used to canonically establish order of records in cases where dates may be ambiguous
    #[serde(rename = "record_id")]
    pub record_id: i32,
    /// start_date string
    #[serde(rename = "start_date")]
    pub start_date: String,
}

impl GetCharactersCharacterIdCorporationhistory200Ok {
    /// 200 ok object
    pub fn new(corporation_id: i32, record_id: i32, start_date: String) -> GetCharactersCharacterIdCorporationhistory200Ok {
        GetCharactersCharacterIdCorporationhistory200Ok {
            corporation_id,
            is_deleted: None,
            record_id,
            start_date,
        }
    }
}


