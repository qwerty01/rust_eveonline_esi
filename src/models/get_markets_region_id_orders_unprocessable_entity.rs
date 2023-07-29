/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMarketsRegionIdOrdersUnprocessableEntity : Unprocessable entity



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMarketsRegionIdOrdersUnprocessableEntity {
    /// Unprocessable entity message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetMarketsRegionIdOrdersUnprocessableEntity {
    /// Unprocessable entity
    pub fn new() -> GetMarketsRegionIdOrdersUnprocessableEntity {
        GetMarketsRegionIdOrdersUnprocessableEntity {
            error: None,
        }
    }
}


