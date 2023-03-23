/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServiceUnavailable : Service unavailable model

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceUnavailable {
    /// Service unavailable message
    #[serde(rename = "error")]
    pub error: String,
}

impl ServiceUnavailable {
    /// Service unavailable model
    pub fn new(error: String) -> ServiceUnavailable {
        ServiceUnavailable { error }
    }
}
