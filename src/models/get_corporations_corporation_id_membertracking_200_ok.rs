/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdMembertracking200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdMembertracking200Ok {
    /// base_id integer
    #[serde(rename = "base_id", skip_serializing_if = "Option::is_none")]
    pub base_id: Option<i32>,
    /// character_id integer
    #[serde(rename = "character_id")]
    pub character_id: i32,
    /// location_id integer
    #[serde(rename = "location_id", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    /// logoff_date string
    #[serde(rename = "logoff_date", skip_serializing_if = "Option::is_none")]
    pub logoff_date: Option<String>,
    /// logon_date string
    #[serde(rename = "logon_date", skip_serializing_if = "Option::is_none")]
    pub logon_date: Option<String>,
    /// ship_type_id integer
    #[serde(rename = "ship_type_id", skip_serializing_if = "Option::is_none")]
    pub ship_type_id: Option<i32>,
    /// start_date string
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl GetCorporationsCorporationIdMembertracking200Ok {
    /// 200 ok object
    pub fn new(character_id: i32) -> GetCorporationsCorporationIdMembertracking200Ok {
        GetCorporationsCorporationIdMembertracking200Ok {
            base_id: None,
            character_id,
            location_id: None,
            logoff_date: None,
            logon_date: None,
            ship_type_id: None,
            start_date: None,
        }
    }
}


