/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseTypesTypeIdDogmaAttribute : dogma_attribute object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdDogmaAttribute {
    /// attribute_id integer
    #[serde(rename = "attribute_id")]
    pub attribute_id: i32,
    /// value number
    #[serde(rename = "value")]
    pub value: f32,
}

impl GetUniverseTypesTypeIdDogmaAttribute {
    /// dogma_attribute object
    pub fn new(attribute_id: i32, value: f32) -> GetUniverseTypesTypeIdDogmaAttribute {
        GetUniverseTypesTypeIdDogmaAttribute {
            attribute_id,
            value,
        }
    }
}


