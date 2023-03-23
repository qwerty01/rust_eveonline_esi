/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetInsurancePricesLevel : level object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetInsurancePricesLevel {
    /// cost number
    #[serde(rename = "cost")]
    pub cost: f32,
    /// Localized insurance level
    #[serde(rename = "name")]
    pub name: String,
    /// payout number
    #[serde(rename = "payout")]
    pub payout: f32,
}

impl GetInsurancePricesLevel {
    /// level object
    pub fn new(cost: f32, name: String, payout: f32) -> GetInsurancePricesLevel {
        GetInsurancePricesLevel { cost, name, payout }
    }
}
