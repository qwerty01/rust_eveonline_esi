/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsSystem : system object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUniverseIdsSystem {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsSystem {
    /// system object
    pub fn new() -> PostUniverseIdsSystem {
        PostUniverseIdsSystem {
            id: None,
            name: None,
        }
    }
}
