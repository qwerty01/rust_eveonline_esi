/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetIndustrySystems200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetIndustrySystems200Ok {
    /// cost_indices array
    #[serde(rename = "cost_indices")]
    pub cost_indices: Vec<crate::models::GetIndustrySystemsCostIndice>,
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
}

impl GetIndustrySystems200Ok {
    /// 200 ok object
    pub fn new(
        cost_indices: Vec<crate::models::GetIndustrySystemsCostIndice>,
        solar_system_id: i32,
    ) -> GetIndustrySystems200Ok {
        GetIndustrySystems200Ok {
            cost_indices,
            solar_system_id,
        }
    }
}
