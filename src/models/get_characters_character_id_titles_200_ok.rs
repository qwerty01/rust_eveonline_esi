/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdTitles200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdTitles200Ok {
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// title_id integer
    #[serde(rename = "title_id", skip_serializing_if = "Option::is_none")]
    pub title_id: Option<i32>,
}

impl GetCharactersCharacterIdTitles200Ok {
    /// 200 ok object
    pub fn new() -> GetCharactersCharacterIdTitles200Ok {
        GetCharactersCharacterIdTitles200Ok {
            name: None,
            title_id: None,
        }
    }
}
