/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetSovereigntyCampaigns200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetSovereigntyCampaigns200Ok {
    /// Score for all attacking parties, only present in Defense Events.
    #[serde(rename = "attackers_score", skip_serializing_if = "Option::is_none")]
    pub attackers_score: Option<f32>,
    /// Unique ID for this campaign.
    #[serde(rename = "campaign_id")]
    pub campaign_id: i32,
    /// The constellation in which the campaign will take place.
    #[serde(rename = "constellation_id")]
    pub constellation_id: i32,
    /// Defending alliance, only present in Defense Events
    #[serde(rename = "defender_id", skip_serializing_if = "Option::is_none")]
    pub defender_id: Option<i32>,
    /// Score for the defending alliance, only present in Defense Events.
    #[serde(rename = "defender_score", skip_serializing_if = "Option::is_none")]
    pub defender_score: Option<f32>,
    /// Type of event this campaign is for. tcu_defense, ihub_defense and station_defense are referred to as \"Defense Events\", station_freeport as \"Freeport Events\".
    #[serde(rename = "event_type")]
    pub event_type: EventType,
    /// Alliance participating and their respective scores, only present in Freeport Events.
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::GetSovereigntyCampaignsParticipant>>,
    /// The solar system the structure is located in.
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// Time the event is scheduled to start.
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// The structure item ID that is related to this campaign.
    #[serde(rename = "structure_id")]
    pub structure_id: i64,
}

impl GetSovereigntyCampaigns200Ok {
    /// 200 ok object
    pub fn new(
        campaign_id: i32,
        constellation_id: i32,
        event_type: EventType,
        solar_system_id: i32,
        start_time: String,
        structure_id: i64,
    ) -> GetSovereigntyCampaigns200Ok {
        GetSovereigntyCampaigns200Ok {
            attackers_score: None,
            campaign_id,
            constellation_id,
            defender_id: None,
            defender_score: None,
            event_type,
            participants: None,
            solar_system_id,
            start_time,
            structure_id,
        }
    }
}

/// Type of event this campaign is for. tcu_defense, ihub_defense and station_defense are referred to as \"Defense Events\", station_freeport as \"Freeport Events\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "tcu_defense")]
    TcuDefense,
    #[serde(rename = "ihub_defense")]
    IhubDefense,
    #[serde(rename = "station_defense")]
    StationDefense,
    #[serde(rename = "station_freeport")]
    StationFreeport,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::TcuDefense
    }
}
