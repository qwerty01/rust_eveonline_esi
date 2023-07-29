/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdBlueprints200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdBlueprints200Ok {
    /// Unique ID for this item.
    #[serde(rename = "item_id")]
    pub item_id: i64,
    /// Type of the location_id
    #[serde(rename = "location_flag")]
    pub location_flag: LocationFlag,
    /// References a station, a ship or an item_id if this blueprint is located within a container. If the return value is an item_id, then the Character AssetList API must be queried to find the container using the given item_id to determine the correct location of the Blueprint.
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// Material Efficiency Level of the blueprint.
    #[serde(rename = "material_efficiency")]
    pub material_efficiency: i32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1 is an original and -2 is a copy. It can be a positive integer if it is a stack of blueprint originals fresh from the market (e.g. no activities performed on them yet).
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original.
    #[serde(rename = "runs")]
    pub runs: i32,
    /// Time Efficiency Level of the blueprint.
    #[serde(rename = "time_efficiency")]
    pub time_efficiency: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdBlueprints200Ok {
    /// 200 ok object
    pub fn new(item_id: i64, location_flag: LocationFlag, location_id: i64, material_efficiency: i32, quantity: i32, runs: i32, time_efficiency: i32, type_id: i32) -> GetCharactersCharacterIdBlueprints200Ok {
        GetCharactersCharacterIdBlueprints200Ok {
            item_id,
            location_flag,
            location_id,
            material_efficiency,
            quantity,
            runs,
            time_efficiency,
            type_id,
        }
    }
}

/// Type of the location_id
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationFlag {
    #[serde(rename = "AutoFit")]
    AutoFit,
    #[serde(rename = "Cargo")]
    Cargo,
    #[serde(rename = "CorpseBay")]
    CorpseBay,
    #[serde(rename = "DroneBay")]
    DroneBay,
    #[serde(rename = "FleetHangar")]
    FleetHangar,
    #[serde(rename = "Deliveries")]
    Deliveries,
    #[serde(rename = "HiddenModifiers")]
    HiddenModifiers,
    #[serde(rename = "Hangar")]
    Hangar,
    #[serde(rename = "HangarAll")]
    HangarAll,
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
    #[serde(rename = "AssetSafety")]
    AssetSafety,
    #[serde(rename = "Locked")]
    Locked,
    #[serde(rename = "Unlocked")]
    Unlocked,
    #[serde(rename = "Implant")]
    Implant,
    #[serde(rename = "QuafeBay")]
    QuafeBay,
    #[serde(rename = "RigSlot0")]
    RigSlot0,
    #[serde(rename = "RigSlot1")]
    RigSlot1,
    #[serde(rename = "RigSlot2")]
    RigSlot2,
    #[serde(rename = "RigSlot3")]
    RigSlot3,
    #[serde(rename = "RigSlot4")]
    RigSlot4,
    #[serde(rename = "RigSlot5")]
    RigSlot5,
    #[serde(rename = "RigSlot6")]
    RigSlot6,
    #[serde(rename = "RigSlot7")]
    RigSlot7,
    #[serde(rename = "ShipHangar")]
    ShipHangar,
    #[serde(rename = "SpecializedFuelBay")]
    SpecializedFuelBay,
    #[serde(rename = "SpecializedOreHold")]
    SpecializedOreHold,
    #[serde(rename = "SpecializedGasHold")]
    SpecializedGasHold,
    #[serde(rename = "SpecializedMineralHold")]
    SpecializedMineralHold,
    #[serde(rename = "SpecializedSalvageHold")]
    SpecializedSalvageHold,
    #[serde(rename = "SpecializedShipHold")]
    SpecializedShipHold,
    #[serde(rename = "SpecializedSmallShipHold")]
    SpecializedSmallShipHold,
    #[serde(rename = "SpecializedMediumShipHold")]
    SpecializedMediumShipHold,
    #[serde(rename = "SpecializedLargeShipHold")]
    SpecializedLargeShipHold,
    #[serde(rename = "SpecializedIndustrialShipHold")]
    SpecializedIndustrialShipHold,
    #[serde(rename = "SpecializedAmmoHold")]
    SpecializedAmmoHold,
    #[serde(rename = "SpecializedCommandCenterHold")]
    SpecializedCommandCenterHold,
    #[serde(rename = "SpecializedPlanetaryCommoditiesHold")]
    SpecializedPlanetaryCommoditiesHold,
    #[serde(rename = "SpecializedMaterialBay")]
    SpecializedMaterialBay,
    #[serde(rename = "SubSystemSlot0")]
    SubSystemSlot0,
    #[serde(rename = "SubSystemSlot1")]
    SubSystemSlot1,
    #[serde(rename = "SubSystemSlot2")]
    SubSystemSlot2,
    #[serde(rename = "SubSystemSlot3")]
    SubSystemSlot3,
    #[serde(rename = "SubSystemSlot4")]
    SubSystemSlot4,
    #[serde(rename = "SubSystemSlot5")]
    SubSystemSlot5,
    #[serde(rename = "SubSystemSlot6")]
    SubSystemSlot6,
    #[serde(rename = "SubSystemSlot7")]
    SubSystemSlot7,
    #[serde(rename = "FighterBay")]
    FighterBay,
    #[serde(rename = "FighterTube0")]
    FighterTube0,
    #[serde(rename = "FighterTube1")]
    FighterTube1,
    #[serde(rename = "FighterTube2")]
    FighterTube2,
    #[serde(rename = "FighterTube3")]
    FighterTube3,
    #[serde(rename = "FighterTube4")]
    FighterTube4,
    #[serde(rename = "Module")]
    Module,
}

impl Default for LocationFlag {
    fn default() -> LocationFlag {
        Self::AutoFit
    }
}

