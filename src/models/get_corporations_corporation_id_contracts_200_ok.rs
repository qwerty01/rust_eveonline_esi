/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdContracts200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContracts200Ok {
    /// Who will accept the contract
    #[serde(rename = "acceptor_id")]
    pub acceptor_id: i32,
    /// ID to whom the contract is assigned, can be corporation or character ID
    #[serde(rename = "assignee_id")]
    pub assignee_id: i32,
    /// To whom the contract is available
    #[serde(rename = "availability")]
    pub availability: Availability,
    /// Buyout price (for Auctions only)
    #[serde(rename = "buyout", skip_serializing_if = "Option::is_none")]
    pub buyout: Option<f64>,
    /// Collateral price (for Couriers only)
    #[serde(rename = "collateral", skip_serializing_if = "Option::is_none")]
    pub collateral: Option<f64>,
    /// contract_id integer
    #[serde(rename = "contract_id")]
    pub contract_id: i32,
    /// Date of confirmation of contract
    #[serde(rename = "date_accepted", skip_serializing_if = "Option::is_none")]
    pub date_accepted: Option<String>,
    /// Date of completed of contract
    #[serde(rename = "date_completed", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<String>,
    /// Expiration date of the contract
    #[serde(rename = "date_expired")]
    pub date_expired: String,
    /// Сreation date of the contract
    #[serde(rename = "date_issued")]
    pub date_issued: String,
    /// Number of days to perform the contract
    #[serde(rename = "days_to_complete", skip_serializing_if = "Option::is_none")]
    pub days_to_complete: Option<i32>,
    /// End location ID (for Couriers contract)
    #[serde(rename = "end_location_id", skip_serializing_if = "Option::is_none")]
    pub end_location_id: Option<i64>,
    /// true if the contract was issued on behalf of the issuer's corporation
    #[serde(rename = "for_corporation")]
    pub for_corporation: bool,
    /// Character's corporation ID for the issuer
    #[serde(rename = "issuer_corporation_id")]
    pub issuer_corporation_id: i32,
    /// Character ID for the issuer
    #[serde(rename = "issuer_id")]
    pub issuer_id: i32,
    /// Price of contract (for ItemsExchange and Auctions)
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Remuneration for contract (for Couriers only)
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<f64>,
    /// Start location ID (for Couriers contract)
    #[serde(rename = "start_location_id", skip_serializing_if = "Option::is_none")]
    pub start_location_id: Option<i64>,
    /// Status of the the contract
    #[serde(rename = "status")]
    pub status: Status,
    /// Title of the contract
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Type of the contract
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Volume of items in the contract
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}

impl GetCorporationsCorporationIdContracts200Ok {
    /// 200 ok object
    pub fn new(acceptor_id: i32, assignee_id: i32, availability: Availability, contract_id: i32, date_expired: String, date_issued: String, for_corporation: bool, issuer_corporation_id: i32, issuer_id: i32, status: Status, r#type: RHashType) -> GetCorporationsCorporationIdContracts200Ok {
        GetCorporationsCorporationIdContracts200Ok {
            acceptor_id,
            assignee_id,
            availability,
            buyout: None,
            collateral: None,
            contract_id,
            date_accepted: None,
            date_completed: None,
            date_expired,
            date_issued,
            days_to_complete: None,
            end_location_id: None,
            for_corporation,
            issuer_corporation_id,
            issuer_id,
            price: None,
            reward: None,
            start_location_id: None,
            status,
            title: None,
            r#type,
            volume: None,
        }
    }
}

/// To whom the contract is available
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "alliance")]
    Alliance,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::Public
    }
}
/// Status of the the contract
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "outstanding")]
    Outstanding,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished_issuer")]
    FinishedIssuer,
    #[serde(rename = "finished_contractor")]
    FinishedContractor,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "reversed")]
    Reversed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Outstanding
    }
}
/// Type of the contract
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "item_exchange")]
    ItemExchange,
    #[serde(rename = "auction")]
    Auction,
    #[serde(rename = "courier")]
    Courier,
    #[serde(rename = "loan")]
    Loan,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Unknown
    }
}

