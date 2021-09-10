/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetKillmailsKillmailIdKillmailHashOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashOk {
    /// attackers array
    #[serde(rename = "attackers")]
    pub attackers: Vec<crate::models::GetKillmailsKillmailIdKillmailHashAttacker>,
    /// ID of the killmail
    #[serde(rename = "killmail_id")]
    pub killmail_id: i32,
    /// Time that the victim was killed and the killmail generated 
    #[serde(rename = "killmail_time")]
    pub killmail_time: String,
    /// Moon if the kill took place at one
    #[serde(rename = "moon_id", skip_serializing_if = "Option::is_none")]
    pub moon_id: Option<i32>,
    /// Solar system that the kill took place in 
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    #[serde(rename = "victim")]
    pub victim: Box<crate::models::GetKillmailsKillmailIdKillmailHashVictim>,
    /// War if the killmail is generated in relation to an official war 
    #[serde(rename = "war_id", skip_serializing_if = "Option::is_none")]
    pub war_id: Option<i32>,
}

impl GetKillmailsKillmailIdKillmailHashOk {
    /// 200 ok object
    pub fn new(attackers: Vec<crate::models::GetKillmailsKillmailIdKillmailHashAttacker>, killmail_id: i32, killmail_time: String, solar_system_id: i32, victim: crate::models::GetKillmailsKillmailIdKillmailHashVictim) -> GetKillmailsKillmailIdKillmailHashOk {
        GetKillmailsKillmailIdKillmailHashOk {
            attackers,
            killmail_id,
            killmail_time,
            moon_id: None,
            solar_system_id,
            victim: Box::new(victim),
            war_id: None,
        }
    }
}


