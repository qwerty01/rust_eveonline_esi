/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSystemsSystemIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseSystemsSystemIdOk {
    /// The constellation this solar system is in
    #[serde(rename = "constellation_id")]
    pub constellation_id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// planets array
    #[serde(rename = "planets", skip_serializing_if = "Option::is_none")]
    pub planets: Option<Vec<crate::models::GetUniverseSystemsSystemIdPlanet>>,
    #[serde(rename = "position")]
    pub position: Box<crate::models::GetUniverseSystemsSystemIdPosition>,
    /// security_class string
    #[serde(rename = "security_class", skip_serializing_if = "Option::is_none")]
    pub security_class: Option<String>,
    /// security_status number
    #[serde(rename = "security_status")]
    pub security_status: f32,
    /// star_id integer
    #[serde(rename = "star_id", skip_serializing_if = "Option::is_none")]
    pub star_id: Option<i32>,
    /// stargates array
    #[serde(rename = "stargates", skip_serializing_if = "Option::is_none")]
    pub stargates: Option<Vec<i32>>,
    /// stations array
    #[serde(rename = "stations", skip_serializing_if = "Option::is_none")]
    pub stations: Option<Vec<i32>>,
    /// system_id integer
    #[serde(rename = "system_id")]
    pub system_id: i32,
}

impl GetUniverseSystemsSystemIdOk {
    /// 200 ok object
    pub fn new(
        constellation_id: i32,
        name: String,
        position: crate::models::GetUniverseSystemsSystemIdPosition,
        security_status: f32,
        system_id: i32,
    ) -> GetUniverseSystemsSystemIdOk {
        GetUniverseSystemsSystemIdOk {
            constellation_id,
            name,
            planets: None,
            position: Box::new(position),
            security_class: None,
            security_status,
            star_id: None,
            stargates: None,
            stations: None,
            system_id,
        }
    }
}
