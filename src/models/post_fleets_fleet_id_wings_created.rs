/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostFleetsFleetIdWingsCreated : 201 created object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFleetsFleetIdWingsCreated {
    /// The wing_id of the newly created wing
    #[serde(rename = "wing_id")]
    pub wing_id: i64,
}

impl PostFleetsFleetIdWingsCreated {
    /// 201 created object
    pub fn new(wing_id: i64) -> PostFleetsFleetIdWingsCreated {
        PostFleetsFleetIdWingsCreated {
            wing_id,
        }
    }
}


