/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdStarbasesStarbaseIdFuel : fuel object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
    /// fuel object
    pub fn new(quantity: i32, type_id: i32) -> GetCorporationsCorporationIdStarbasesStarbaseIdFuel {
        GetCorporationsCorporationIdStarbasesStarbaseIdFuel { quantity, type_id }
    }
}
