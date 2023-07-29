/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsStation : station object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUniverseIdsStation {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsStation {
    /// station object
    pub fn new() -> PostUniverseIdsStation {
        PostUniverseIdsStation {
            id: None,
            name: None,
        }
    }
}


