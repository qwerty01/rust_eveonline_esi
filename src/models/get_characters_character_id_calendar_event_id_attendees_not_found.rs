/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdCalendarEventIdAttendeesNotFound : Not found



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdCalendarEventIdAttendeesNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetCharactersCharacterIdCalendarEventIdAttendeesNotFound {
    /// Not found
    pub fn new() -> GetCharactersCharacterIdCalendarEventIdAttendeesNotFound {
        GetCharactersCharacterIdCalendarEventIdAttendeesNotFound {
            error: None,
        }
    }
}

