/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseStationsStationIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseStationsStationIdOk {
    /// max_dockable_ship_volume number
    #[serde(rename = "max_dockable_ship_volume")]
    pub max_dockable_ship_volume: f32,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// office_rental_cost number
    #[serde(rename = "office_rental_cost")]
    pub office_rental_cost: f32,
    /// ID of the corporation that controls this station
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<i32>,
    #[serde(rename = "position")]
    pub position: Box<crate::models::GetUniverseStationsStationIdPosition>,
    /// race_id integer
    #[serde(rename = "race_id", skip_serializing_if = "Option::is_none")]
    pub race_id: Option<i32>,
    /// reprocessing_efficiency number
    #[serde(rename = "reprocessing_efficiency")]
    pub reprocessing_efficiency: f32,
    /// reprocessing_stations_take number
    #[serde(rename = "reprocessing_stations_take")]
    pub reprocessing_stations_take: f32,
    /// services array
    #[serde(rename = "services")]
    pub services: Vec<Services>,
    /// station_id integer
    #[serde(rename = "station_id")]
    pub station_id: i32,
    /// The solar system this station is in
    #[serde(rename = "system_id")]
    pub system_id: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetUniverseStationsStationIdOk {
    /// 200 ok object
    pub fn new(max_dockable_ship_volume: f32, name: String, office_rental_cost: f32, position: crate::models::GetUniverseStationsStationIdPosition, reprocessing_efficiency: f32, reprocessing_stations_take: f32, services: Vec<Services>, station_id: i32, system_id: i32, type_id: i32) -> GetUniverseStationsStationIdOk {
        GetUniverseStationsStationIdOk {
            max_dockable_ship_volume,
            name,
            office_rental_cost,
            owner: None,
            position: Box::new(position),
            race_id: None,
            reprocessing_efficiency,
            reprocessing_stations_take,
            services,
            station_id,
            system_id,
            type_id,
        }
    }
}

/// services array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Services {
    #[serde(rename = "bounty-missions")]
    BountyMissions,
    #[serde(rename = "assasination-missions")]
    AssasinationMissions,
    #[serde(rename = "courier-missions")]
    CourierMissions,
    #[serde(rename = "interbus")]
    Interbus,
    #[serde(rename = "reprocessing-plant")]
    ReprocessingPlant,
    #[serde(rename = "refinery")]
    Refinery,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "black-market")]
    BlackMarket,
    #[serde(rename = "stock-exchange")]
    StockExchange,
    #[serde(rename = "cloning")]
    Cloning,
    #[serde(rename = "surgery")]
    Surgery,
    #[serde(rename = "dna-therapy")]
    DnaTherapy,
    #[serde(rename = "repair-facilities")]
    RepairFacilities,
    #[serde(rename = "factory")]
    Factory,
    #[serde(rename = "labratory")]
    Labratory,
    #[serde(rename = "gambling")]
    Gambling,
    #[serde(rename = "fitting")]
    Fitting,
    #[serde(rename = "paintshop")]
    Paintshop,
    #[serde(rename = "news")]
    News,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "insurance")]
    Insurance,
    #[serde(rename = "docking")]
    Docking,
    #[serde(rename = "office-rental")]
    OfficeRental,
    #[serde(rename = "jump-clone-facility")]
    JumpCloneFacility,
    #[serde(rename = "loyalty-point-store")]
    LoyaltyPointStore,
    #[serde(rename = "navy-offices")]
    NavyOffices,
    #[serde(rename = "security-offices")]
    SecurityOffices,
}

impl Default for Services {
    fn default() -> Services {
        Self::BountyMissions
    }
}

