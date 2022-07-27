/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsAlliance : alliance object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUniverseIdsAlliance {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsAlliance {
    /// alliance object
    pub fn new() -> PostUniverseIdsAlliance {
        PostUniverseIdsAlliance {
            id: None,
            name: None,
        }
    }
}
