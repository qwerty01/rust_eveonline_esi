/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdContractsContractIdItems200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdContractsContractIdItems200Ok {
    /// true if the contract issuer has submitted this item with the contract, false if the isser is asking for this item in the contract
    #[serde(rename = "is_included")]
    pub is_included: bool,
    /// is_singleton boolean
    #[serde(rename = "is_singleton")]
    pub is_singleton: bool,
    /// Number of items in the stack
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// -1 indicates that the item is a singleton (non-stackable). If the item happens to be a Blueprint, -1 is an Original and -2 is a Blueprint Copy
    #[serde(rename = "raw_quantity", skip_serializing_if = "Option::is_none")]
    pub raw_quantity: Option<i32>,
    /// Unique ID for the item
    #[serde(rename = "record_id")]
    pub record_id: i64,
    /// Type ID for item
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdContractsContractIdItems200Ok {
    /// 200 ok object
    pub fn new(
        is_included: bool,
        is_singleton: bool,
        quantity: i32,
        record_id: i64,
        type_id: i32,
    ) -> GetCharactersCharacterIdContractsContractIdItems200Ok {
        GetCharactersCharacterIdContractsContractIdItems200Ok {
            is_included,
            is_singleton,
            quantity,
            raw_quantity: None,
            record_id,
            type_id,
        }
    }
}
