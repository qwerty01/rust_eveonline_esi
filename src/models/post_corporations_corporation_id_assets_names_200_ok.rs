/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostCorporationsCorporationIdAssetsNames200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCorporationsCorporationIdAssetsNames200Ok {
    /// item_id integer
    #[serde(rename = "item_id")]
    pub item_id: i64,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
}

impl PostCorporationsCorporationIdAssetsNames200Ok {
    /// 200 ok object
    pub fn new(item_id: i64, name: String) -> PostCorporationsCorporationIdAssetsNames200Ok {
        PostCorporationsCorporationIdAssetsNames200Ok {
            item_id,
            name,
        }
    }
}


