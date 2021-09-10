/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMarketsGroupsMarketGroupIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMarketsGroupsMarketGroupIdOk {
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// market_group_id integer
    #[serde(rename = "market_group_id")]
    pub market_group_id: i32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// parent_group_id integer
    #[serde(rename = "parent_group_id", skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<i32>,
    /// types array
    #[serde(rename = "types")]
    pub types: Vec<i32>,
}

impl GetMarketsGroupsMarketGroupIdOk {
    /// 200 ok object
    pub fn new(description: String, market_group_id: i32, name: String, types: Vec<i32>) -> GetMarketsGroupsMarketGroupIdOk {
        GetMarketsGroupsMarketGroupIdOk {
            description,
            market_group_id,
            name,
            parent_group_id: None,
            types,
        }
    }
}

