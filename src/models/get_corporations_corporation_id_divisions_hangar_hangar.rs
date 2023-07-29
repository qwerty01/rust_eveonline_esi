/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdDivisionsHangarHangar : hangar object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdDivisionsHangarHangar {
    /// division integer
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCorporationsCorporationIdDivisionsHangarHangar {
    /// hangar object
    pub fn new() -> GetCorporationsCorporationIdDivisionsHangarHangar {
        GetCorporationsCorporationIdDivisionsHangarHangar {
            division: None,
            name: None,
        }
    }
}


