/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetKillmailsKillmailIdKillmailHashVictim : victim object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashVictim {
    /// alliance_id integer
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// character_id integer
    #[serde(rename = "character_id", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
    /// How much total damage was taken by the victim 
    #[serde(rename = "damage_taken")]
    pub damage_taken: i32,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// items array
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::GetKillmailsKillmailIdKillmailHashItem>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::GetKillmailsKillmailIdKillmailHashPosition>>,
    /// The ship that the victim was piloting and was destroyed 
    #[serde(rename = "ship_type_id")]
    pub ship_type_id: i32,
}

impl GetKillmailsKillmailIdKillmailHashVictim {
    /// victim object
    pub fn new(damage_taken: i32, ship_type_id: i32) -> GetKillmailsKillmailIdKillmailHashVictim {
        GetKillmailsKillmailIdKillmailHashVictim {
            alliance_id: None,
            character_id: None,
            corporation_id: None,
            damage_taken,
            faction_id: None,
            items: None,
            position: None,
            ship_type_id,
        }
    }
}


