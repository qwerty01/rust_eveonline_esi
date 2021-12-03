/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetContractsPublicBidsContractIdForbidden : Forbidden

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContractsPublicBidsContractIdForbidden {
    /// Forbidden message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetContractsPublicBidsContractIdForbidden {
    /// Forbidden
    pub fn new() -> GetContractsPublicBidsContractIdForbidden {
        GetContractsPublicBidsContractIdForbidden { error: None }
    }
}
