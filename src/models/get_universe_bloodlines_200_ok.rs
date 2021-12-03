/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseBloodlines200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseBloodlines200Ok {
    /// bloodline_id integer
    #[serde(rename = "bloodline_id")]
    pub bloodline_id: i32,
    /// charisma integer
    #[serde(rename = "charisma")]
    pub charisma: i32,
    /// corporation_id integer
    #[serde(rename = "corporation_id")]
    pub corporation_id: i32,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// intelligence integer
    #[serde(rename = "intelligence")]
    pub intelligence: i32,
    /// memory integer
    #[serde(rename = "memory")]
    pub memory: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// perception integer
    #[serde(rename = "perception")]
    pub perception: i32,
    /// race_id integer
    #[serde(rename = "race_id")]
    pub race_id: i32,
    /// ship_type_id integer
    #[serde(rename = "ship_type_id")]
    pub ship_type_id: Option<i32>,
    /// willpower integer
    #[serde(rename = "willpower")]
    pub willpower: i32,
}

impl GetUniverseBloodlines200Ok {
    /// 200 ok object
    pub fn new(
        bloodline_id: i32,
        charisma: i32,
        corporation_id: i32,
        description: String,
        intelligence: i32,
        memory: i32,
        name: String,
        perception: i32,
        race_id: i32,
        ship_type_id: Option<i32>,
        willpower: i32,
    ) -> GetUniverseBloodlines200Ok {
        GetUniverseBloodlines200Ok {
            bloodline_id,
            charisma,
            corporation_id,
            description,
            intelligence,
            memory,
            name,
            perception,
            race_id,
            ship_type_id,
            willpower,
        }
    }
}
