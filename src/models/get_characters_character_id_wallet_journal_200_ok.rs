/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdWalletJournal200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdWalletJournal200Ok {
    /// The amount of ISK given or taken from the wallet as a result of the given transaction. Positive when ISK is deposited into the wallet and negative when ISK is withdrawn
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Wallet balance after transaction occurred
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// An ID that gives extra context to the particular transaction. Because of legacy reasons the context is completely different per ref_type and means different things. It is also possible to not have a context_id
    #[serde(rename = "context_id", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<i64>,
    /// The type of the given context_id if present
    #[serde(rename = "context_id_type", skip_serializing_if = "Option::is_none")]
    pub context_id_type: Option<ContextIdType>,
    /// Date and time of transaction
    #[serde(rename = "date")]
    pub date: String,
    /// The reason for the transaction, mirrors what is seen in the client
    #[serde(rename = "description")]
    pub description: String,
    /// The id of the first party involved in the transaction. This attribute has no consistency and is different or non existant for particular ref_types. The description attribute will help make sense of what this attribute means. For more info about the given ID it can be dropped into the /universe/names/ ESI route to determine its type and name
    #[serde(rename = "first_party_id", skip_serializing_if = "Option::is_none")]
    pub first_party_id: Option<i32>,
    /// Unique journal reference ID
    #[serde(rename = "id")]
    pub id: i64,
    /// The user stated reason for the transaction. Only applies to some ref_types
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// \"The transaction type for the given. transaction. Different transaction types will populate different attributes.\"
    #[serde(rename = "ref_type")]
    pub ref_type: RefType,
    /// The id of the second party involved in the transaction. This attribute has no consistency and is different or non existant for particular ref_types. The description attribute will help make sense of what this attribute means. For more info about the given ID it can be dropped into the /universe/names/ ESI route to determine its type and name
    #[serde(rename = "second_party_id", skip_serializing_if = "Option::is_none")]
    pub second_party_id: Option<i32>,
    /// Tax amount received. Only applies to tax related transactions
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    /// The corporation ID receiving any tax paid. Only applies to tax related transactions
    #[serde(rename = "tax_receiver_id", skip_serializing_if = "Option::is_none")]
    pub tax_receiver_id: Option<i32>,
}

impl GetCharactersCharacterIdWalletJournal200Ok {
    /// 200 ok object
    pub fn new(
        date: String,
        description: String,
        id: i64,
        ref_type: RefType,
    ) -> GetCharactersCharacterIdWalletJournal200Ok {
        GetCharactersCharacterIdWalletJournal200Ok {
            amount: None,
            balance: None,
            context_id: None,
            context_id_type: None,
            date,
            description,
            first_party_id: None,
            id,
            reason: None,
            ref_type,
            second_party_id: None,
            tax: None,
            tax_receiver_id: None,
        }
    }
}

/// The type of the given context_id if present
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContextIdType {
    #[serde(rename = "structure_id")]
    StructureId,
    #[serde(rename = "station_id")]
    StationId,
    #[serde(rename = "market_transaction_id")]
    MarketTransactionId,
    #[serde(rename = "character_id")]
    CharacterId,
    #[serde(rename = "corporation_id")]
    CorporationId,
    #[serde(rename = "alliance_id")]
    AllianceId,
    #[serde(rename = "eve_system")]
    EveSystem,
    #[serde(rename = "industry_job_id")]
    IndustryJobId,
    #[serde(rename = "contract_id")]
    ContractId,
    #[serde(rename = "planet_id")]
    PlanetId,
    #[serde(rename = "system_id")]
    SystemId,
    #[serde(rename = "type_id")]
    TypeId,
}

impl Default for ContextIdType {
    fn default() -> ContextIdType {
        Self::StructureId
    }
}
/// \"The transaction type for the given. transaction. Different transaction types will populate different attributes.\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RefType {
    #[serde(rename = "acceleration_gate_fee")]
    AccelerationGateFee,
    #[serde(rename = "advertisement_listing_fee")]
    AdvertisementListingFee,
    #[serde(rename = "agent_donation")]
    AgentDonation,
    #[serde(rename = "agent_location_services")]
    AgentLocationServices,
    #[serde(rename = "agent_miscellaneous")]
    AgentMiscellaneous,
    #[serde(rename = "agent_mission_collateral_paid")]
    AgentMissionCollateralPaid,
    #[serde(rename = "agent_mission_collateral_refunded")]
    AgentMissionCollateralRefunded,
    #[serde(rename = "agent_mission_reward")]
    AgentMissionReward,
    #[serde(rename = "agent_mission_reward_corporation_tax")]
    AgentMissionRewardCorporationTax,
    #[serde(rename = "agent_mission_time_bonus_reward")]
    AgentMissionTimeBonusReward,
    #[serde(rename = "agent_mission_time_bonus_reward_corporation_tax")]
    AgentMissionTimeBonusRewardCorporationTax,
    #[serde(rename = "agent_security_services")]
    AgentSecurityServices,
    #[serde(rename = "agent_services_rendered")]
    AgentServicesRendered,
    #[serde(rename = "agents_preward")]
    AgentsPreward,
    #[serde(rename = "alliance_maintainance_fee")]
    AllianceMaintainanceFee,
    #[serde(rename = "alliance_registration_fee")]
    AllianceRegistrationFee,
    #[serde(rename = "asset_safety_recovery_tax")]
    AssetSafetyRecoveryTax,
    #[serde(rename = "bounty")]
    Bounty,
    #[serde(rename = "bounty_prize")]
    BountyPrize,
    #[serde(rename = "bounty_prize_corporation_tax")]
    BountyPrizeCorporationTax,
    #[serde(rename = "bounty_prizes")]
    BountyPrizes,
    #[serde(rename = "bounty_reimbursement")]
    BountyReimbursement,
    #[serde(rename = "bounty_surcharge")]
    BountySurcharge,
    #[serde(rename = "brokers_fee")]
    BrokersFee,
    #[serde(rename = "clone_activation")]
    CloneActivation,
    #[serde(rename = "clone_transfer")]
    CloneTransfer,
    #[serde(rename = "contraband_fine")]
    ContrabandFine,
    #[serde(rename = "contract_auction_bid")]
    ContractAuctionBid,
    #[serde(rename = "contract_auction_bid_corp")]
    ContractAuctionBidCorp,
    #[serde(rename = "contract_auction_bid_refund")]
    ContractAuctionBidRefund,
    #[serde(rename = "contract_auction_sold")]
    ContractAuctionSold,
    #[serde(rename = "contract_brokers_fee")]
    ContractBrokersFee,
    #[serde(rename = "contract_brokers_fee_corp")]
    ContractBrokersFeeCorp,
    #[serde(rename = "contract_collateral")]
    ContractCollateral,
    #[serde(rename = "contract_collateral_deposited_corp")]
    ContractCollateralDepositedCorp,
    #[serde(rename = "contract_collateral_payout")]
    ContractCollateralPayout,
    #[serde(rename = "contract_collateral_refund")]
    ContractCollateralRefund,
    #[serde(rename = "contract_deposit")]
    ContractDeposit,
    #[serde(rename = "contract_deposit_corp")]
    ContractDepositCorp,
    #[serde(rename = "contract_deposit_refund")]
    ContractDepositRefund,
    #[serde(rename = "contract_deposit_sales_tax")]
    ContractDepositSalesTax,
    #[serde(rename = "contract_price")]
    ContractPrice,
    #[serde(rename = "contract_price_payment_corp")]
    ContractPricePaymentCorp,
    #[serde(rename = "contract_reversal")]
    ContractReversal,
    #[serde(rename = "contract_reward")]
    ContractReward,
    #[serde(rename = "contract_reward_deposited")]
    ContractRewardDeposited,
    #[serde(rename = "contract_reward_deposited_corp")]
    ContractRewardDepositedCorp,
    #[serde(rename = "contract_reward_refund")]
    ContractRewardRefund,
    #[serde(rename = "contract_sales_tax")]
    ContractSalesTax,
    #[serde(rename = "copying")]
    Copying,
    #[serde(rename = "corporate_reward_payout")]
    CorporateRewardPayout,
    #[serde(rename = "corporate_reward_tax")]
    CorporateRewardTax,
    #[serde(rename = "corporation_account_withdrawal")]
    CorporationAccountWithdrawal,
    #[serde(rename = "corporation_bulk_payment")]
    CorporationBulkPayment,
    #[serde(rename = "corporation_dividend_payment")]
    CorporationDividendPayment,
    #[serde(rename = "corporation_liquidation")]
    CorporationLiquidation,
    #[serde(rename = "corporation_logo_change_cost")]
    CorporationLogoChangeCost,
    #[serde(rename = "corporation_payment")]
    CorporationPayment,
    #[serde(rename = "corporation_registration_fee")]
    CorporationRegistrationFee,
    #[serde(rename = "courier_mission_escrow")]
    CourierMissionEscrow,
    #[serde(rename = "cspa")]
    Cspa,
    #[serde(rename = "cspaofflinerefund")]
    Cspaofflinerefund,
    #[serde(rename = "daily_challenge_reward")]
    DailyChallengeReward,
    #[serde(rename = "datacore_fee")]
    DatacoreFee,
    #[serde(rename = "dna_modification_fee")]
    DnaModificationFee,
    #[serde(rename = "docking_fee")]
    DockingFee,
    #[serde(rename = "duel_wager_escrow")]
    DuelWagerEscrow,
    #[serde(rename = "duel_wager_payment")]
    DuelWagerPayment,
    #[serde(rename = "duel_wager_refund")]
    DuelWagerRefund,
    #[serde(rename = "ess_escrow_transfer")]
    EssEscrowTransfer,
    #[serde(rename = "external_trade_delivery")]
    ExternalTradeDelivery,
    #[serde(rename = "external_trade_freeze")]
    ExternalTradeFreeze,
    #[serde(rename = "external_trade_thaw")]
    ExternalTradeThaw,
    #[serde(rename = "factory_slot_rental_fee")]
    FactorySlotRentalFee,
    #[serde(rename = "flux_payout")]
    FluxPayout,
    #[serde(rename = "flux_tax")]
    FluxTax,
    #[serde(rename = "flux_ticket_repayment")]
    FluxTicketRepayment,
    #[serde(rename = "flux_ticket_sale")]
    FluxTicketSale,
    #[serde(rename = "gm_cash_transfer")]
    GmCashTransfer,
    #[serde(rename = "industry_job_tax")]
    IndustryJobTax,
    #[serde(rename = "infrastructure_hub_maintenance")]
    InfrastructureHubMaintenance,
    #[serde(rename = "inheritance")]
    Inheritance,
    #[serde(rename = "insurance")]
    Insurance,
    #[serde(rename = "item_trader_payment")]
    ItemTraderPayment,
    #[serde(rename = "jump_clone_activation_fee")]
    JumpCloneActivationFee,
    #[serde(rename = "jump_clone_installation_fee")]
    JumpCloneInstallationFee,
    #[serde(rename = "kill_right_fee")]
    KillRightFee,
    #[serde(rename = "lp_store")]
    LpStore,
    #[serde(rename = "manufacturing")]
    Manufacturing,
    #[serde(rename = "market_escrow")]
    MarketEscrow,
    #[serde(rename = "market_fine_paid")]
    MarketFinePaid,
    #[serde(rename = "market_provider_tax")]
    MarketProviderTax,
    #[serde(rename = "market_transaction")]
    MarketTransaction,
    #[serde(rename = "medal_creation")]
    MedalCreation,
    #[serde(rename = "medal_issued")]
    MedalIssued,
    #[serde(rename = "milestone_reward_payment")]
    MilestoneRewardPayment,
    #[serde(rename = "mission_completion")]
    MissionCompletion,
    #[serde(rename = "mission_cost")]
    MissionCost,
    #[serde(rename = "mission_expiration")]
    MissionExpiration,
    #[serde(rename = "mission_reward")]
    MissionReward,
    #[serde(rename = "office_rental_fee")]
    OfficeRentalFee,
    #[serde(rename = "operation_bonus")]
    OperationBonus,
    #[serde(rename = "opportunity_reward")]
    OpportunityReward,
    #[serde(rename = "planetary_construction")]
    PlanetaryConstruction,
    #[serde(rename = "planetary_export_tax")]
    PlanetaryExportTax,
    #[serde(rename = "planetary_import_tax")]
    PlanetaryImportTax,
    #[serde(rename = "player_donation")]
    PlayerDonation,
    #[serde(rename = "player_trading")]
    PlayerTrading,
    #[serde(rename = "project_discovery_reward")]
    ProjectDiscoveryReward,
    #[serde(rename = "project_discovery_tax")]
    ProjectDiscoveryTax,
    #[serde(rename = "reaction")]
    Reaction,
    #[serde(rename = "redeemed_isk_token")]
    RedeemedIskToken,
    #[serde(rename = "release_of_impounded_property")]
    ReleaseOfImpoundedProperty,
    #[serde(rename = "repair_bill")]
    RepairBill,
    #[serde(rename = "reprocessing_tax")]
    ReprocessingTax,
    #[serde(rename = "researching_material_productivity")]
    ResearchingMaterialProductivity,
    #[serde(rename = "researching_technology")]
    ResearchingTechnology,
    #[serde(rename = "researching_time_productivity")]
    ResearchingTimeProductivity,
    #[serde(rename = "resource_wars_reward")]
    ResourceWarsReward,
    #[serde(rename = "reverse_engineering")]
    ReverseEngineering,
    #[serde(rename = "season_challenge_reward")]
    SeasonChallengeReward,
    #[serde(rename = "security_processing_fee")]
    SecurityProcessingFee,
    #[serde(rename = "shares")]
    Shares,
    #[serde(rename = "skill_purchase")]
    SkillPurchase,
    #[serde(rename = "sovereignity_bill")]
    SovereignityBill,
    #[serde(rename = "store_purchase")]
    StorePurchase,
    #[serde(rename = "store_purchase_refund")]
    StorePurchaseRefund,
    #[serde(rename = "structure_gate_jump")]
    StructureGateJump,
    #[serde(rename = "transaction_tax")]
    TransactionTax,
    #[serde(rename = "upkeep_adjustment_fee")]
    UpkeepAdjustmentFee,
    #[serde(rename = "war_ally_contract")]
    WarAllyContract,
    #[serde(rename = "war_fee")]
    WarFee,
    #[serde(rename = "war_fee_surrender")]
    WarFeeSurrender,
}

impl Default for RefType {
    fn default() -> RefType {
        Self::AccelerationGateFee
    }
}
