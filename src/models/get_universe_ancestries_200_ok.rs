/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseAncestries200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseAncestries200Ok {
    /// The bloodline associated with this ancestry
    #[serde(rename = "bloodline_id")]
    pub bloodline_id: i32,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// icon_id integer
    #[serde(rename = "icon_id", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<i32>,
    /// id integer
    #[serde(rename = "id")]
    pub id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// short_description string
    #[serde(rename = "short_description", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
}

impl GetUniverseAncestries200Ok {
    /// 200 ok object
    pub fn new(bloodline_id: i32, description: String, id: i32, name: String) -> GetUniverseAncestries200Ok {
        GetUniverseAncestries200Ok {
            bloodline_id,
            description,
            icon_id: None,
            id,
            name,
            short_description: None,
        }
    }
}


