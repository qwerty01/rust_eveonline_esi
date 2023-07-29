/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdMedals200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdMedals200Ok {
    /// created_at string
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// ID of the character who created this medal
    #[serde(rename = "creator_id")]
    pub creator_id: i32,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// medal_id integer
    #[serde(rename = "medal_id")]
    pub medal_id: i32,
    /// title string
    #[serde(rename = "title")]
    pub title: String,
}

impl GetCorporationsCorporationIdMedals200Ok {
    /// 200 ok object
    pub fn new(created_at: String, creator_id: i32, description: String, medal_id: i32, title: String) -> GetCorporationsCorporationIdMedals200Ok {
        GetCorporationsCorporationIdMedals200Ok {
            created_at,
            creator_id,
            description,
            medal_id,
            title,
        }
    }
}


