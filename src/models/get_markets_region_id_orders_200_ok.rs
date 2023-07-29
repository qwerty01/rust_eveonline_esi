/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetMarketsRegionIdOrders200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMarketsRegionIdOrders200Ok {
    /// duration integer
    #[serde(rename = "duration")]
    pub duration: i32,
    /// is_buy_order boolean
    #[serde(rename = "is_buy_order")]
    pub is_buy_order: bool,
    /// issued string
    #[serde(rename = "issued")]
    pub issued: String,
    /// location_id integer
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// min_volume integer
    #[serde(rename = "min_volume")]
    pub min_volume: i32,
    /// order_id integer
    #[serde(rename = "order_id")]
    pub order_id: i64,
    /// price number
    #[serde(rename = "price")]
    pub price: f64,
    /// range string
    #[serde(rename = "range")]
    pub range: Range,
    /// The solar system this order was placed
    #[serde(rename = "system_id")]
    pub system_id: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
    /// volume_remain integer
    #[serde(rename = "volume_remain")]
    pub volume_remain: i32,
    /// volume_total integer
    #[serde(rename = "volume_total")]
    pub volume_total: i32,
}

impl GetMarketsRegionIdOrders200Ok {
    /// 200 ok object
    pub fn new(duration: i32, is_buy_order: bool, issued: String, location_id: i64, min_volume: i32, order_id: i64, price: f64, range: Range, system_id: i32, type_id: i32, volume_remain: i32, volume_total: i32) -> GetMarketsRegionIdOrders200Ok {
        GetMarketsRegionIdOrders200Ok {
            duration,
            is_buy_order,
            issued,
            location_id,
            min_volume,
            order_id,
            price,
            range,
            system_id,
            type_id,
            volume_remain,
            volume_total,
        }
    }
}

/// range string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Range {
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "solarsystem")]
    Solarsystem,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "20")]
    Variant20,
    #[serde(rename = "30")]
    Variant30,
    #[serde(rename = "40")]
    Variant40,
}

impl Default for Range {
    fn default() -> Range {
        Self::Station
    }
}

