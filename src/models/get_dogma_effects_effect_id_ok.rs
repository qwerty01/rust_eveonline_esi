/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetDogmaEffectsEffectIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDogmaEffectsEffectIdOk {
    /// description string
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// disallow_auto_repeat boolean
    #[serde(rename = "disallow_auto_repeat", skip_serializing_if = "Option::is_none")]
    pub disallow_auto_repeat: Option<bool>,
    /// discharge_attribute_id integer
    #[serde(rename = "discharge_attribute_id", skip_serializing_if = "Option::is_none")]
    pub discharge_attribute_id: Option<i32>,
    /// display_name string
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// duration_attribute_id integer
    #[serde(rename = "duration_attribute_id", skip_serializing_if = "Option::is_none")]
    pub duration_attribute_id: Option<i32>,
    /// effect_category integer
    #[serde(rename = "effect_category", skip_serializing_if = "Option::is_none")]
    pub effect_category: Option<i32>,
    /// effect_id integer
    #[serde(rename = "effect_id")]
    pub effect_id: i32,
    /// electronic_chance boolean
    #[serde(rename = "electronic_chance", skip_serializing_if = "Option::is_none")]
    pub electronic_chance: Option<bool>,
    /// falloff_attribute_id integer
    #[serde(rename = "falloff_attribute_id", skip_serializing_if = "Option::is_none")]
    pub falloff_attribute_id: Option<i32>,
    /// icon_id integer
    #[serde(rename = "icon_id", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<i32>,
    /// is_assistance boolean
    #[serde(rename = "is_assistance", skip_serializing_if = "Option::is_none")]
    pub is_assistance: Option<bool>,
    /// is_offensive boolean
    #[serde(rename = "is_offensive", skip_serializing_if = "Option::is_none")]
    pub is_offensive: Option<bool>,
    /// is_warp_safe boolean
    #[serde(rename = "is_warp_safe", skip_serializing_if = "Option::is_none")]
    pub is_warp_safe: Option<bool>,
    /// modifiers array
    #[serde(rename = "modifiers", skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<crate::models::GetDogmaEffectsEffectIdModifier>>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// post_expression integer
    #[serde(rename = "post_expression", skip_serializing_if = "Option::is_none")]
    pub post_expression: Option<i32>,
    /// pre_expression integer
    #[serde(rename = "pre_expression", skip_serializing_if = "Option::is_none")]
    pub pre_expression: Option<i32>,
    /// published boolean
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// range_attribute_id integer
    #[serde(rename = "range_attribute_id", skip_serializing_if = "Option::is_none")]
    pub range_attribute_id: Option<i32>,
    /// range_chance boolean
    #[serde(rename = "range_chance", skip_serializing_if = "Option::is_none")]
    pub range_chance: Option<bool>,
    /// tracking_speed_attribute_id integer
    #[serde(rename = "tracking_speed_attribute_id", skip_serializing_if = "Option::is_none")]
    pub tracking_speed_attribute_id: Option<i32>,
}

impl GetDogmaEffectsEffectIdOk {
    /// 200 ok object
    pub fn new(effect_id: i32) -> GetDogmaEffectsEffectIdOk {
        GetDogmaEffectsEffectIdOk {
            description: None,
            disallow_auto_repeat: None,
            discharge_attribute_id: None,
            display_name: None,
            duration_attribute_id: None,
            effect_category: None,
            effect_id,
            electronic_chance: None,
            falloff_attribute_id: None,
            icon_id: None,
            is_assistance: None,
            is_offensive: None,
            is_warp_safe: None,
            modifiers: None,
            name: None,
            post_expression: None,
            pre_expression: None,
            published: None,
            range_attribute_id: None,
            range_chance: None,
            tracking_speed_attribute_id: None,
        }
    }
}


