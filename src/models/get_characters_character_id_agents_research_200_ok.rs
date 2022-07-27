/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdAgentsResearch200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdAgentsResearch200Ok {
    /// agent_id integer
    #[serde(rename = "agent_id")]
    pub agent_id: i32,
    /// points_per_day number
    #[serde(rename = "points_per_day")]
    pub points_per_day: f32,
    /// remainder_points number
    #[serde(rename = "remainder_points")]
    pub remainder_points: f32,
    /// skill_type_id integer
    #[serde(rename = "skill_type_id")]
    pub skill_type_id: i32,
    /// started_at string
    #[serde(rename = "started_at")]
    pub started_at: String,
}

impl GetCharactersCharacterIdAgentsResearch200Ok {
    /// 200 ok object
    pub fn new(
        agent_id: i32,
        points_per_day: f32,
        remainder_points: f32,
        skill_type_id: i32,
        started_at: String,
    ) -> GetCharactersCharacterIdAgentsResearch200Ok {
        GetCharactersCharacterIdAgentsResearch200Ok {
            agent_id,
            points_per_day,
            remainder_points,
            skill_type_id,
            started_at,
        }
    }
}
