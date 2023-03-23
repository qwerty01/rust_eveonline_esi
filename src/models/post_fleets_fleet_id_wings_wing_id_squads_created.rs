/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostFleetsFleetIdWingsWingIdSquadsCreated : 201 created object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostFleetsFleetIdWingsWingIdSquadsCreated {
    /// The squad_id of the newly created squad
    #[serde(rename = "squad_id")]
    pub squad_id: i64,
}

impl PostFleetsFleetIdWingsWingIdSquadsCreated {
    /// 201 created object
    pub fn new(squad_id: i64) -> PostFleetsFleetIdWingsWingIdSquadsCreated {
        PostFleetsFleetIdWingsWingIdSquadsCreated { squad_id }
    }
}
