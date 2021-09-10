/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdContractsContractIdItemsError520 : Error 520



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContractsContractIdItemsError520 {
    /// Error 520 message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetCorporationsCorporationIdContractsContractIdItemsError520 {
    /// Error 520
    pub fn new() -> GetCorporationsCorporationIdContractsContractIdItemsError520 {
        GetCorporationsCorporationIdContractsContractIdItemsError520 {
            error: None,
        }
    }
}


