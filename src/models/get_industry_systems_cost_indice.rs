/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetIndustrySystemsCostIndice : cost_indice object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetIndustrySystemsCostIndice {
    /// activity string
    #[serde(rename = "activity")]
    pub activity: Activity,
    /// cost_index number
    #[serde(rename = "cost_index")]
    pub cost_index: f32,
}

impl GetIndustrySystemsCostIndice {
    /// cost_indice object
    pub fn new(activity: Activity, cost_index: f32) -> GetIndustrySystemsCostIndice {
        GetIndustrySystemsCostIndice {
            activity,
            cost_index,
        }
    }
}

/// activity string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Activity {
    #[serde(rename = "copying")]
    Copying,
    #[serde(rename = "duplicating")]
    Duplicating,
    #[serde(rename = "invention")]
    Invention,
    #[serde(rename = "manufacturing")]
    Manufacturing,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "reaction")]
    Reaction,
    #[serde(rename = "researching_material_efficiency")]
    ResearchingMaterialEfficiency,
    #[serde(rename = "researching_technology")]
    ResearchingTechnology,
    #[serde(rename = "researching_time_efficiency")]
    ResearchingTimeEfficiency,
    #[serde(rename = "reverse_engineering")]
    ReverseEngineering,
}

impl Default for Activity {
    fn default() -> Activity {
        Self::Copying
    }
}
