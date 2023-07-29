/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetKillmailsKillmailIdKillmailHashItem : item object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashItem {
    /// Flag for the location of the item 
    #[serde(rename = "flag")]
    pub flag: i32,
    /// item_type_id integer
    #[serde(rename = "item_type_id")]
    pub item_type_id: i32,
    /// items array
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::GetKillmailsKillmailIdKillmailHashItemsItem>>,
    /// How many of the item were destroyed if any 
    #[serde(rename = "quantity_destroyed", skip_serializing_if = "Option::is_none")]
    pub quantity_destroyed: Option<i64>,
    /// How many of the item were dropped if any 
    #[serde(rename = "quantity_dropped", skip_serializing_if = "Option::is_none")]
    pub quantity_dropped: Option<i64>,
    /// singleton integer
    #[serde(rename = "singleton")]
    pub singleton: i32,
}

impl GetKillmailsKillmailIdKillmailHashItem {
    /// item object
    pub fn new(flag: i32, item_type_id: i32, singleton: i32) -> GetKillmailsKillmailIdKillmailHashItem {
        GetKillmailsKillmailIdKillmailHashItem {
            flag,
            item_type_id,
            items: None,
            quantity_destroyed: None,
            quantity_dropped: None,
            singleton,
        }
    }
}


