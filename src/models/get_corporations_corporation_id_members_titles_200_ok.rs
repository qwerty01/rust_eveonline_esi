/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdMembersTitles200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdMembersTitles200Ok {
    /// character_id integer
    #[serde(rename = "character_id")]
    pub character_id: i32,
    /// A list of title_id
    #[serde(rename = "titles")]
    pub titles: Vec<i32>,
}

impl GetCorporationsCorporationIdMembersTitles200Ok {
    /// 200 ok object
    pub fn new(
        character_id: i32,
        titles: Vec<i32>,
    ) -> GetCorporationsCorporationIdMembersTitles200Ok {
        GetCorporationsCorporationIdMembersTitles200Ok {
            character_id,
            titles,
        }
    }
}
