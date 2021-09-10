/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseSystemsSystemIdPlanet : planet object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseSystemsSystemIdPlanet {
    /// asteroid_belts array
    #[serde(rename = "asteroid_belts", skip_serializing_if = "Option::is_none")]
    pub asteroid_belts: Option<Vec<i32>>,
    /// moons array
    #[serde(rename = "moons", skip_serializing_if = "Option::is_none")]
    pub moons: Option<Vec<i32>>,
    /// planet_id integer
    #[serde(rename = "planet_id")]
    pub planet_id: i32,
}

impl GetUniverseSystemsSystemIdPlanet {
    /// planet object
    pub fn new(planet_id: i32) -> GetUniverseSystemsSystemIdPlanet {
        GetUniverseSystemsSystemIdPlanet {
            asteroid_belts: None,
            moons: None,
            planet_id,
        }
    }
}


