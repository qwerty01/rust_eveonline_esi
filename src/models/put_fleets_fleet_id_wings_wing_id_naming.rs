/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutFleetsFleetIdWingsWingIdNaming : naming object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutFleetsFleetIdWingsWingIdNaming {
    /// name string
    #[serde(rename = "name")]
    pub name: String,
}

impl PutFleetsFleetIdWingsWingIdNaming {
    /// naming object
    pub fn new(name: String) -> PutFleetsFleetIdWingsWingIdNaming {
        PutFleetsFleetIdWingsWingIdNaming {
            name,
        }
    }
}

