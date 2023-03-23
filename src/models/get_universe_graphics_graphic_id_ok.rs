/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseGraphicsGraphicIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseGraphicsGraphicIdOk {
    /// collision_file string
    #[serde(rename = "collision_file", skip_serializing_if = "Option::is_none")]
    pub collision_file: Option<String>,
    /// graphic_file string
    #[serde(rename = "graphic_file", skip_serializing_if = "Option::is_none")]
    pub graphic_file: Option<String>,
    /// graphic_id integer
    #[serde(rename = "graphic_id")]
    pub graphic_id: i32,
    /// icon_folder string
    #[serde(rename = "icon_folder", skip_serializing_if = "Option::is_none")]
    pub icon_folder: Option<String>,
    /// sof_dna string
    #[serde(rename = "sof_dna", skip_serializing_if = "Option::is_none")]
    pub sof_dna: Option<String>,
    /// sof_fation_name string
    #[serde(rename = "sof_fation_name", skip_serializing_if = "Option::is_none")]
    pub sof_fation_name: Option<String>,
    /// sof_hull_name string
    #[serde(rename = "sof_hull_name", skip_serializing_if = "Option::is_none")]
    pub sof_hull_name: Option<String>,
    /// sof_race_name string
    #[serde(rename = "sof_race_name", skip_serializing_if = "Option::is_none")]
    pub sof_race_name: Option<String>,
}

impl GetUniverseGraphicsGraphicIdOk {
    /// 200 ok object
    pub fn new(graphic_id: i32) -> GetUniverseGraphicsGraphicIdOk {
        GetUniverseGraphicsGraphicIdOk {
            collision_file: None,
            graphic_file: None,
            graphic_id,
            icon_folder: None,
            sof_dna: None,
            sof_fation_name: None,
            sof_hull_name: None,
            sof_race_name: None,
        }
    }
}
