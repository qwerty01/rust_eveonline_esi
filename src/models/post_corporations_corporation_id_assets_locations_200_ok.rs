/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostCorporationsCorporationIdAssetsLocations200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostCorporationsCorporationIdAssetsLocations200Ok {
    /// item_id integer
    #[serde(rename = "item_id")]
    pub item_id: i64,
    #[serde(rename = "position")]
    pub position: Box<crate::models::PostCorporationsCorporationIdAssetsLocationsPosition>,
}

impl PostCorporationsCorporationIdAssetsLocations200Ok {
    /// 200 ok object
    pub fn new(
        item_id: i64,
        position: crate::models::PostCorporationsCorporationIdAssetsLocationsPosition,
    ) -> PostCorporationsCorporationIdAssetsLocations200Ok {
        PostCorporationsCorporationIdAssetsLocations200Ok {
            item_id,
            position: Box::new(position),
        }
    }
}
