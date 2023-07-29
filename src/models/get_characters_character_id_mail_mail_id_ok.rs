/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMailMailIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMailMailIdOk {
    /// Mail's body
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// From whom the mail was sent
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    /// Labels attached to the mail
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i32>>,
    /// Whether the mail is flagged as read
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// Recipients of the mail
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<crate::models::GetCharactersCharacterIdMailMailIdRecipient>>,
    /// Mail subject
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// When the mail was sent
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl GetCharactersCharacterIdMailMailIdOk {
    /// 200 ok object
    pub fn new() -> GetCharactersCharacterIdMailMailIdOk {
        GetCharactersCharacterIdMailMailIdOk {
            body: None,
            from: None,
            labels: None,
            read: None,
            recipients: None,
            subject: None,
            timestamp: None,
        }
    }
}


