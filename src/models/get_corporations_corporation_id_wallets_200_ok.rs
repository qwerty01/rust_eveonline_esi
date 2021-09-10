/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdWallets200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdWallets200Ok {
    /// balance number
    #[serde(rename = "balance")]
    pub balance: f64,
    /// division integer
    #[serde(rename = "division")]
    pub division: i32,
}

impl GetCorporationsCorporationIdWallets200Ok {
    /// 200 ok object
    pub fn new(balance: f64, division: i32) -> GetCorporationsCorporationIdWallets200Ok {
        GetCorporationsCorporationIdWallets200Ok {
            balance,
            division,
        }
    }
}


