/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdRolesHistory200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdRolesHistory200Ok {
    /// changed_at string
    #[serde(rename = "changed_at")]
    pub changed_at: String,
    /// The character whose roles are changed
    #[serde(rename = "character_id")]
    pub character_id: i32,
    /// ID of the character who issued this change
    #[serde(rename = "issuer_id")]
    pub issuer_id: i32,
    /// new_roles array
    #[serde(rename = "new_roles")]
    pub new_roles: Vec<NewRoles>,
    /// old_roles array
    #[serde(rename = "old_roles")]
    pub old_roles: Vec<OldRoles>,
    /// role_type string
    #[serde(rename = "role_type")]
    pub role_type: RoleType,
}

impl GetCorporationsCorporationIdRolesHistory200Ok {
    /// 200 ok object
    pub fn new(changed_at: String, character_id: i32, issuer_id: i32, new_roles: NewRoles, old_roles: OldRoles, role_type: RoleType) -> GetCorporationsCorporationIdRolesHistory200Ok {
        GetCorporationsCorporationIdRolesHistory200Ok {
            changed_at,
            character_id,
            issuer_id,
            new_roles,
            old_roles,
            role_type,
        }
    }
}

/// new_roles array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NewRoles {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
/// old_roles array
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OldRoles {
    #[serde(rename = "Account_Take_1")]
    AccountTake1,
    #[serde(rename = "Account_Take_2")]
    AccountTake2,
    #[serde(rename = "Account_Take_3")]
    AccountTake3,
    #[serde(rename = "Account_Take_4")]
    AccountTake4,
    #[serde(rename = "Account_Take_5")]
    AccountTake5,
    #[serde(rename = "Account_Take_6")]
    AccountTake6,
    #[serde(rename = "Account_Take_7")]
    AccountTake7,
    #[serde(rename = "Accountant")]
    Accountant,
    #[serde(rename = "Auditor")]
    Auditor,
    #[serde(rename = "Communications_Officer")]
    CommunicationsOfficer,
    #[serde(rename = "Config_Equipment")]
    ConfigEquipment,
    #[serde(rename = "Config_Starbase_Equipment")]
    ConfigStarbaseEquipment,
    #[serde(rename = "Container_Take_1")]
    ContainerTake1,
    #[serde(rename = "Container_Take_2")]
    ContainerTake2,
    #[serde(rename = "Container_Take_3")]
    ContainerTake3,
    #[serde(rename = "Container_Take_4")]
    ContainerTake4,
    #[serde(rename = "Container_Take_5")]
    ContainerTake5,
    #[serde(rename = "Container_Take_6")]
    ContainerTake6,
    #[serde(rename = "Container_Take_7")]
    ContainerTake7,
    #[serde(rename = "Contract_Manager")]
    ContractManager,
    #[serde(rename = "Diplomat")]
    Diplomat,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Factory_Manager")]
    FactoryManager,
    #[serde(rename = "Fitting_Manager")]
    FittingManager,
    #[serde(rename = "Hangar_Query_1")]
    HangarQuery1,
    #[serde(rename = "Hangar_Query_2")]
    HangarQuery2,
    #[serde(rename = "Hangar_Query_3")]
    HangarQuery3,
    #[serde(rename = "Hangar_Query_4")]
    HangarQuery4,
    #[serde(rename = "Hangar_Query_5")]
    HangarQuery5,
    #[serde(rename = "Hangar_Query_6")]
    HangarQuery6,
    #[serde(rename = "Hangar_Query_7")]
    HangarQuery7,
    #[serde(rename = "Hangar_Take_1")]
    HangarTake1,
    #[serde(rename = "Hangar_Take_2")]
    HangarTake2,
    #[serde(rename = "Hangar_Take_3")]
    HangarTake3,
    #[serde(rename = "Hangar_Take_4")]
    HangarTake4,
    #[serde(rename = "Hangar_Take_5")]
    HangarTake5,
    #[serde(rename = "Hangar_Take_6")]
    HangarTake6,
    #[serde(rename = "Hangar_Take_7")]
    HangarTake7,
    #[serde(rename = "Junior_Accountant")]
    JuniorAccountant,
    #[serde(rename = "Personnel_Manager")]
    PersonnelManager,
    #[serde(rename = "Rent_Factory_Facility")]
    RentFactoryFacility,
    #[serde(rename = "Rent_Office")]
    RentOffice,
    #[serde(rename = "Rent_Research_Facility")]
    RentResearchFacility,
    #[serde(rename = "Security_Officer")]
    SecurityOfficer,
    #[serde(rename = "Starbase_Defense_Operator")]
    StarbaseDefenseOperator,
    #[serde(rename = "Starbase_Fuel_Technician")]
    StarbaseFuelTechnician,
    #[serde(rename = "Station_Manager")]
    StationManager,
    #[serde(rename = "Trader")]
    Trader,
}
/// role_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleType {
    #[serde(rename = "grantable_roles")]
    GrantableRoles,
    #[serde(rename = "grantable_roles_at_base")]
    GrantableRolesAtBase,
    #[serde(rename = "grantable_roles_at_hq")]
    GrantableRolesAtHq,
    #[serde(rename = "grantable_roles_at_other")]
    GrantableRolesAtOther,
    #[serde(rename = "roles")]
    Roles,
    #[serde(rename = "roles_at_base")]
    RolesAtBase,
    #[serde(rename = "roles_at_hq")]
    RolesAtHq,
    #[serde(rename = "roles_at_other")]
    RolesAtOther,
}
