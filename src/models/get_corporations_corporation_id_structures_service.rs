/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdStructuresService : service object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStructuresService {
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// state string
    #[serde(rename = "state")]
    pub state: State,
}

impl GetCorporationsCorporationIdStructuresService {
    /// service object
    pub fn new(name: String, state: State) -> GetCorporationsCorporationIdStructuresService {
        GetCorporationsCorporationIdStructuresService { name, state }
    }
}

/// state string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "cleanup")]
    Cleanup,
}
