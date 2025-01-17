/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutFleetsFleetIdMembersMemberIdNotFound : Not found



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFleetsFleetIdMembersMemberIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PutFleetsFleetIdMembersMemberIdNotFound {
    /// Not found
    pub fn new() -> PutFleetsFleetIdMembersMemberIdNotFound {
        PutFleetsFleetIdMembersMemberIdNotFound {
            error: None,
        }
    }
}


