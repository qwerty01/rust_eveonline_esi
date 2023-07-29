/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutFleetsFleetIdNewSettings : new_settings object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFleetsFleetIdNewSettings {
    /// Should free-move be enabled in the fleet
    #[serde(rename = "is_free_move", skip_serializing_if = "Option::is_none")]
    pub is_free_move: Option<bool>,
    /// New fleet MOTD in CCP flavoured HTML
    #[serde(rename = "motd", skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
}

impl PutFleetsFleetIdNewSettings {
    /// new_settings object
    pub fn new() -> PutFleetsFleetIdNewSettings {
        PutFleetsFleetIdNewSettings {
            is_free_move: None,
            motd: None,
        }
    }
}


