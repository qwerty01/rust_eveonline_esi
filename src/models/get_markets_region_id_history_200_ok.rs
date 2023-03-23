/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetMarketsRegionIdHistory200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetMarketsRegionIdHistory200Ok {
    /// average number
    #[serde(rename = "average")]
    pub average: f64,
    /// The date of this historical statistic entry
    #[serde(rename = "date")]
    pub date: String,
    /// highest number
    #[serde(rename = "highest")]
    pub highest: f64,
    /// lowest number
    #[serde(rename = "lowest")]
    pub lowest: f64,
    /// Total number of orders happened that day
    #[serde(rename = "order_count")]
    pub order_count: i64,
    /// Total
    #[serde(rename = "volume")]
    pub volume: i64,
}

impl GetMarketsRegionIdHistory200Ok {
    /// 200 ok object
    pub fn new(
        average: f64,
        date: String,
        highest: f64,
        lowest: f64,
        order_count: i64,
        volume: i64,
    ) -> GetMarketsRegionIdHistory200Ok {
        GetMarketsRegionIdHistory200Ok {
            average,
            date,
            highest,
            lowest,
            order_count,
            volume,
        }
    }
}
