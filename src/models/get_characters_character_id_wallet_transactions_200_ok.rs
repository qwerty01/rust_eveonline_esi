/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdWalletTransactions200Ok : wallet transaction

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdWalletTransactions200Ok {
    /// client_id integer
    #[serde(rename = "client_id")]
    pub client_id: i32,
    /// Date and time of transaction
    #[serde(rename = "date")]
    pub date: String,
    /// is_buy boolean
    #[serde(rename = "is_buy")]
    pub is_buy: bool,
    /// is_personal boolean
    #[serde(rename = "is_personal")]
    pub is_personal: bool,
    /// journal_ref_id integer
    #[serde(rename = "journal_ref_id")]
    pub journal_ref_id: i64,
    /// location_id integer
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Unique transaction ID
    #[serde(rename = "transaction_id")]
    pub transaction_id: i64,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
    /// Amount paid per unit
    #[serde(rename = "unit_price")]
    pub unit_price: f64,
}

impl GetCharactersCharacterIdWalletTransactions200Ok {
    /// wallet transaction
    pub fn new(
        client_id: i32,
        date: String,
        is_buy: bool,
        is_personal: bool,
        journal_ref_id: i64,
        location_id: i64,
        quantity: i32,
        transaction_id: i64,
        type_id: i32,
        unit_price: f64,
    ) -> GetCharactersCharacterIdWalletTransactions200Ok {
        GetCharactersCharacterIdWalletTransactions200Ok {
            client_id,
            date,
            is_buy,
            is_personal,
            journal_ref_id,
            location_id,
            quantity,
            transaction_id,
            type_id,
            unit_price,
        }
    }
}
