/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseGroupsGroupIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseGroupsGroupIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniverseGroupsGroupIdNotFound {
    /// Not found
    pub fn new() -> GetUniverseGroupsGroupIdNotFound {
        GetUniverseGroupsGroupIdNotFound { error: None }
    }
}
