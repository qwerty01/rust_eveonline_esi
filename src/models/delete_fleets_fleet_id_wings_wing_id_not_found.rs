/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteFleetsFleetIdWingsWingIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteFleetsFleetIdWingsWingIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl DeleteFleetsFleetIdWingsWingIdNotFound {
    /// Not found
    pub fn new() -> DeleteFleetsFleetIdWingsWingIdNotFound {
        DeleteFleetsFleetIdWingsWingIdNotFound { error: None }
    }
}
