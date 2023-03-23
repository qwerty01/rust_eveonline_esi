/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseTypesTypeIdOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdOk {
    /// capacity number
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f32>,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// dogma_attributes array
    #[serde(rename = "dogma_attributes", skip_serializing_if = "Option::is_none")]
    pub dogma_attributes: Option<Vec<crate::models::GetUniverseTypesTypeIdDogmaAttribute>>,
    /// dogma_effects array
    #[serde(rename = "dogma_effects", skip_serializing_if = "Option::is_none")]
    pub dogma_effects: Option<Vec<crate::models::GetUniverseTypesTypeIdDogmaEffect>>,
    /// graphic_id integer
    #[serde(rename = "graphic_id", skip_serializing_if = "Option::is_none")]
    pub graphic_id: Option<i32>,
    /// group_id integer
    #[serde(rename = "group_id")]
    pub group_id: i32,
    /// icon_id integer
    #[serde(rename = "icon_id", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<i32>,
    /// This only exists for types that can be put on the market
    #[serde(rename = "market_group_id", skip_serializing_if = "Option::is_none")]
    pub market_group_id: Option<i32>,
    /// mass number
    #[serde(rename = "mass", skip_serializing_if = "Option::is_none")]
    pub mass: Option<f32>,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// packaged_volume number
    #[serde(rename = "packaged_volume", skip_serializing_if = "Option::is_none")]
    pub packaged_volume: Option<f32>,
    /// portion_size integer
    #[serde(rename = "portion_size", skip_serializing_if = "Option::is_none")]
    pub portion_size: Option<i32>,
    /// published boolean
    #[serde(rename = "published")]
    pub published: bool,
    /// radius number
    #[serde(rename = "radius", skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
    /// volume number
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
}

impl GetUniverseTypesTypeIdOk {
    /// 200 ok object
    pub fn new(
        description: String,
        group_id: i32,
        name: String,
        published: bool,
        type_id: i32,
    ) -> GetUniverseTypesTypeIdOk {
        GetUniverseTypesTypeIdOk {
            capacity: None,
            description,
            dogma_attributes: None,
            dogma_effects: None,
            graphic_id: None,
            group_id,
            icon_id: None,
            market_group_id: None,
            mass: None,
            name,
            packaged_volume: None,
            portion_size: None,
            published,
            radius: None,
            type_id,
            volume: None,
        }
    }
}
