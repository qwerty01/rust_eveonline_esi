/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUiOpenwindowNewmailUnprocessableEntity : Unprocessable entity



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUiOpenwindowNewmailUnprocessableEntity {
    /// Unprocessable entity message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PostUiOpenwindowNewmailUnprocessableEntity {
    /// Unprocessable entity
    pub fn new() -> PostUiOpenwindowNewmailUnprocessableEntity {
        PostUiOpenwindowNewmailUnprocessableEntity {
            error: None,
        }
    }
}


