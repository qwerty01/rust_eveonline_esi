/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdCalendarEventIdOk : Full details of a specific event

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdCalendarEventIdOk {
    /// date string
    #[serde(rename = "date")]
    pub date: String,
    /// Length in minutes
    #[serde(rename = "duration")]
    pub duration: i32,
    /// event_id integer
    #[serde(rename = "event_id")]
    pub event_id: i32,
    /// importance integer
    #[serde(rename = "importance")]
    pub importance: i32,
    /// owner_id integer
    #[serde(rename = "owner_id")]
    pub owner_id: i32,
    /// owner_name string
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    /// owner_type string
    #[serde(rename = "owner_type")]
    pub owner_type: OwnerType,
    /// response string
    #[serde(rename = "response")]
    pub response: String,
    /// text string
    #[serde(rename = "text")]
    pub text: String,
    /// title string
    #[serde(rename = "title")]
    pub title: String,
}

impl GetCharactersCharacterIdCalendarEventIdOk {
    /// Full details of a specific event
    pub fn new(
        date: String,
        duration: i32,
        event_id: i32,
        importance: i32,
        owner_id: i32,
        owner_name: String,
        owner_type: OwnerType,
        response: String,
        text: String,
        title: String,
    ) -> GetCharactersCharacterIdCalendarEventIdOk {
        GetCharactersCharacterIdCalendarEventIdOk {
            date,
            duration,
            event_id,
            importance,
            owner_id,
            owner_name,
            owner_type,
            response,
            text,
            title,
        }
    }
}

/// owner_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OwnerType {
    #[serde(rename = "eve_server")]
    EveServer,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "faction")]
    Faction,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "alliance")]
    Alliance,
}
