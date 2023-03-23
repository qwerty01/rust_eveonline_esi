/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostCharactersCharacterIdFittingsItem : item object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdFittingsItem {
    /// Fitting location for the item. Entries placed in 'Invalid' will be discarded. If this leaves the fitting with nothing, it will cause an error.
    #[serde(rename = "flag")]
    pub flag: Flag,
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl PostCharactersCharacterIdFittingsItem {
    /// item object
    pub fn new(flag: Flag, quantity: i32, type_id: i32) -> PostCharactersCharacterIdFittingsItem {
        PostCharactersCharacterIdFittingsItem {
            flag,
            quantity,
            type_id,
        }
    }
}

/// Fitting location for the item. Entries placed in 'Invalid' will be discarded. If this leaves the fitting with nothing, it will cause an error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Flag {
    #[serde(rename = "Cargo")]
    Cargo,
    #[serde(rename = "DroneBay")]
    DroneBay,
    #[serde(rename = "FighterBay")]
    FighterBay,
    #[serde(rename = "HiSlot0")]
    HiSlot0,
    #[serde(rename = "HiSlot1")]
    HiSlot1,
    #[serde(rename = "HiSlot2")]
    HiSlot2,
    #[serde(rename = "HiSlot3")]
    HiSlot3,
    #[serde(rename = "HiSlot4")]
    HiSlot4,
    #[serde(rename = "HiSlot5")]
    HiSlot5,
    #[serde(rename = "HiSlot6")]
    HiSlot6,
    #[serde(rename = "HiSlot7")]
    HiSlot7,
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "LoSlot0")]
    LoSlot0,
    #[serde(rename = "LoSlot1")]
    LoSlot1,
    #[serde(rename = "LoSlot2")]
    LoSlot2,
    #[serde(rename = "LoSlot3")]
    LoSlot3,
    #[serde(rename = "LoSlot4")]
    LoSlot4,
    #[serde(rename = "LoSlot5")]
    LoSlot5,
    #[serde(rename = "LoSlot6")]
    LoSlot6,
    #[serde(rename = "LoSlot7")]
    LoSlot7,
    #[serde(rename = "MedSlot0")]
    MedSlot0,
    #[serde(rename = "MedSlot1")]
    MedSlot1,
    #[serde(rename = "MedSlot2")]
    MedSlot2,
    #[serde(rename = "MedSlot3")]
    MedSlot3,
    #[serde(rename = "MedSlot4")]
    MedSlot4,
    #[serde(rename = "MedSlot5")]
    MedSlot5,
    #[serde(rename = "MedSlot6")]
    MedSlot6,
    #[serde(rename = "MedSlot7")]
    MedSlot7,
    #[serde(rename = "RigSlot0")]
    RigSlot0,
    #[serde(rename = "RigSlot1")]
    RigSlot1,
    #[serde(rename = "RigSlot2")]
    RigSlot2,
    #[serde(rename = "ServiceSlot0")]
    ServiceSlot0,
    #[serde(rename = "ServiceSlot1")]
    ServiceSlot1,
    #[serde(rename = "ServiceSlot2")]
    ServiceSlot2,
    #[serde(rename = "ServiceSlot3")]
    ServiceSlot3,
    #[serde(rename = "ServiceSlot4")]
    ServiceSlot4,
    #[serde(rename = "ServiceSlot5")]
    ServiceSlot5,
    #[serde(rename = "ServiceSlot6")]
    ServiceSlot6,
    #[serde(rename = "ServiceSlot7")]
    ServiceSlot7,
    #[serde(rename = "SubSystemSlot0")]
    SubSystemSlot0,
    #[serde(rename = "SubSystemSlot1")]
    SubSystemSlot1,
    #[serde(rename = "SubSystemSlot2")]
    SubSystemSlot2,
    #[serde(rename = "SubSystemSlot3")]
    SubSystemSlot3,
}

impl Default for Flag {
    fn default() -> Flag {
        Self::Cargo
    }
}
