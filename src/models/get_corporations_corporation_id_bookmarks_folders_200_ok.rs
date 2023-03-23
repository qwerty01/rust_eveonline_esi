/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdBookmarksFolders200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdBookmarksFolders200Ok {
    /// creator_id integer
    #[serde(rename = "creator_id", skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i32>,
    /// folder_id integer
    #[serde(rename = "folder_id")]
    pub folder_id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
}

impl GetCorporationsCorporationIdBookmarksFolders200Ok {
    /// 200 ok object
    pub fn new(folder_id: i32, name: String) -> GetCorporationsCorporationIdBookmarksFolders200Ok {
        GetCorporationsCorporationIdBookmarksFolders200Ok {
            creator_id: None,
            folder_id,
            name,
        }
    }
}
