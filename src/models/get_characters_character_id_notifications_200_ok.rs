/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdNotifications200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdNotifications200Ok {
    /// is_read boolean
    #[serde(rename = "is_read", skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
    /// notification_id integer
    #[serde(rename = "notification_id")]
    pub notification_id: i64,
    /// sender_id integer
    #[serde(rename = "sender_id")]
    pub sender_id: i32,
    /// sender_type string
    #[serde(rename = "sender_type")]
    pub sender_type: SenderType,
    /// text string
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// timestamp string
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// type string
    #[serde(rename = "type")]
    pub _type: Type,
}

impl GetCharactersCharacterIdNotifications200Ok {
    /// 200 ok object
    pub fn new(
        notification_id: i64,
        sender_id: i32,
        sender_type: SenderType,
        timestamp: String,
        _type: Type,
    ) -> GetCharactersCharacterIdNotifications200Ok {
        GetCharactersCharacterIdNotifications200Ok {
            is_read: None,
            notification_id,
            sender_id,
            sender_type,
            text: None,
            timestamp,
            _type,
        }
    }
}

/// sender_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SenderType {
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "faction")]
    Faction,
    #[serde(rename = "other")]
    Other,
}

impl Default for SenderType {
    fn default() -> SenderType {
        Self::Character
    }
}
/// type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AcceptedAlly")]
    AcceptedAlly,
    #[serde(rename = "AcceptedSurrender")]
    AcceptedSurrender,
    #[serde(rename = "AgentRetiredTrigravian")]
    AgentRetiredTrigravian,
    #[serde(rename = "AllAnchoringMsg")]
    AllAnchoringMsg,
    #[serde(rename = "AllMaintenanceBillMsg")]
    AllMaintenanceBillMsg,
    #[serde(rename = "AllStrucInvulnerableMsg")]
    AllStrucInvulnerableMsg,
    #[serde(rename = "AllStructVulnerableMsg")]
    AllStructVulnerableMsg,
    #[serde(rename = "AllWarCorpJoinedAllianceMsg")]
    AllWarCorpJoinedAllianceMsg,
    #[serde(rename = "AllWarDeclaredMsg")]
    AllWarDeclaredMsg,
    #[serde(rename = "AllWarInvalidatedMsg")]
    AllWarInvalidatedMsg,
    #[serde(rename = "AllWarRetractedMsg")]
    AllWarRetractedMsg,
    #[serde(rename = "AllWarSurrenderMsg")]
    AllWarSurrenderMsg,
    #[serde(rename = "AllianceCapitalChanged")]
    AllianceCapitalChanged,
    #[serde(rename = "AllianceWarDeclaredV2")]
    AllianceWarDeclaredV2,
    #[serde(rename = "AllyContractCancelled")]
    AllyContractCancelled,
    #[serde(rename = "AllyJoinedWarAggressorMsg")]
    AllyJoinedWarAggressorMsg,
    #[serde(rename = "AllyJoinedWarAllyMsg")]
    AllyJoinedWarAllyMsg,
    #[serde(rename = "AllyJoinedWarDefenderMsg")]
    AllyJoinedWarDefenderMsg,
    #[serde(rename = "BattlePunishFriendlyFire")]
    BattlePunishFriendlyFire,
    #[serde(rename = "BillOutOfMoneyMsg")]
    BillOutOfMoneyMsg,
    #[serde(rename = "BillPaidCorpAllMsg")]
    BillPaidCorpAllMsg,
    #[serde(rename = "BountyClaimMsg")]
    BountyClaimMsg,
    #[serde(rename = "BountyESSShared")]
    BountyESSShared,
    #[serde(rename = "BountyESSTaken")]
    BountyESSTaken,
    #[serde(rename = "BountyPlacedAlliance")]
    BountyPlacedAlliance,
    #[serde(rename = "BountyPlacedChar")]
    BountyPlacedChar,
    #[serde(rename = "BountyPlacedCorp")]
    BountyPlacedCorp,
    #[serde(rename = "BountyYourBountyClaimed")]
    BountyYourBountyClaimed,
    #[serde(rename = "BuddyConnectContactAdd")]
    BuddyConnectContactAdd,
    #[serde(rename = "CharAppAcceptMsg")]
    CharAppAcceptMsg,
    #[serde(rename = "CharAppRejectMsg")]
    CharAppRejectMsg,
    #[serde(rename = "CharAppWithdrawMsg")]
    CharAppWithdrawMsg,
    #[serde(rename = "CharLeftCorpMsg")]
    CharLeftCorpMsg,
    #[serde(rename = "CharMedalMsg")]
    CharMedalMsg,
    #[serde(rename = "CharTerminationMsg")]
    CharTerminationMsg,
    #[serde(rename = "CloneActivationMsg")]
    CloneActivationMsg,
    #[serde(rename = "CloneActivationMsg2")]
    CloneActivationMsg2,
    #[serde(rename = "CloneMovedMsg")]
    CloneMovedMsg,
    #[serde(rename = "CloneRevokedMsg1")]
    CloneRevokedMsg1,
    #[serde(rename = "CloneRevokedMsg2")]
    CloneRevokedMsg2,
    #[serde(rename = "CombatOperationFinished")]
    CombatOperationFinished,
    #[serde(rename = "ContactAdd")]
    ContactAdd,
    #[serde(rename = "ContactEdit")]
    ContactEdit,
    #[serde(rename = "ContainerPasswordMsg")]
    ContainerPasswordMsg,
    #[serde(rename = "ContractRegionChangedToPochven")]
    ContractRegionChangedToPochven,
    #[serde(rename = "CorpAllBillMsg")]
    CorpAllBillMsg,
    #[serde(rename = "CorpAppAcceptMsg")]
    CorpAppAcceptMsg,
    #[serde(rename = "CorpAppInvitedMsg")]
    CorpAppInvitedMsg,
    #[serde(rename = "CorpAppNewMsg")]
    CorpAppNewMsg,
    #[serde(rename = "CorpAppRejectCustomMsg")]
    CorpAppRejectCustomMsg,
    #[serde(rename = "CorpAppRejectMsg")]
    CorpAppRejectMsg,
    #[serde(rename = "CorpBecameWarEligible")]
    CorpBecameWarEligible,
    #[serde(rename = "CorpDividendMsg")]
    CorpDividendMsg,
    #[serde(rename = "CorpFriendlyFireDisableTimerCompleted")]
    CorpFriendlyFireDisableTimerCompleted,
    #[serde(rename = "CorpFriendlyFireDisableTimerStarted")]
    CorpFriendlyFireDisableTimerStarted,
    #[serde(rename = "CorpFriendlyFireEnableTimerCompleted")]
    CorpFriendlyFireEnableTimerCompleted,
    #[serde(rename = "CorpFriendlyFireEnableTimerStarted")]
    CorpFriendlyFireEnableTimerStarted,
    #[serde(rename = "CorpKicked")]
    CorpKicked,
    #[serde(rename = "CorpLiquidationMsg")]
    CorpLiquidationMsg,
    #[serde(rename = "CorpNewCEOMsg")]
    CorpNewCEOMsg,
    #[serde(rename = "CorpNewsMsg")]
    CorpNewsMsg,
    #[serde(rename = "CorpNoLongerWarEligible")]
    CorpNoLongerWarEligible,
    #[serde(rename = "CorpOfficeExpirationMsg")]
    CorpOfficeExpirationMsg,
    #[serde(rename = "CorpStructLostMsg")]
    CorpStructLostMsg,
    #[serde(rename = "CorpTaxChangeMsg")]
    CorpTaxChangeMsg,
    #[serde(rename = "CorpVoteCEORevokedMsg")]
    CorpVoteCEORevokedMsg,
    #[serde(rename = "CorpVoteMsg")]
    CorpVoteMsg,
    #[serde(rename = "CorpWarDeclaredMsg")]
    CorpWarDeclaredMsg,
    #[serde(rename = "CorpWarDeclaredV2")]
    CorpWarDeclaredV2,
    #[serde(rename = "CorpWarFightingLegalMsg")]
    CorpWarFightingLegalMsg,
    #[serde(rename = "CorpWarInvalidatedMsg")]
    CorpWarInvalidatedMsg,
    #[serde(rename = "CorpWarRetractedMsg")]
    CorpWarRetractedMsg,
    #[serde(rename = "CorpWarSurrenderMsg")]
    CorpWarSurrenderMsg,
    #[serde(rename = "CustomsMsg")]
    CustomsMsg,
    #[serde(rename = "DeclareWar")]
    DeclareWar,
    #[serde(rename = "DistrictAttacked")]
    DistrictAttacked,
    #[serde(rename = "DustAppAcceptedMsg")]
    DustAppAcceptedMsg,
    #[serde(rename = "ESSMainBankLink")]
    ESSMainBankLink,
    #[serde(rename = "EntosisCaptureStarted")]
    EntosisCaptureStarted,
    #[serde(rename = "ExpertSystemExpired")]
    ExpertSystemExpired,
    #[serde(rename = "ExpertSystemExpiryImminent")]
    ExpertSystemExpiryImminent,
    #[serde(rename = "FWAllianceKickMsg")]
    FWAllianceKickMsg,
    #[serde(rename = "FWAllianceWarningMsg")]
    FWAllianceWarningMsg,
    #[serde(rename = "FWCharKickMsg")]
    FWCharKickMsg,
    #[serde(rename = "FWCharRankGainMsg")]
    FWCharRankGainMsg,
    #[serde(rename = "FWCharRankLossMsg")]
    FWCharRankLossMsg,
    #[serde(rename = "FWCharWarningMsg")]
    FWCharWarningMsg,
    #[serde(rename = "FWCorpJoinMsg")]
    FWCorpJoinMsg,
    #[serde(rename = "FWCorpKickMsg")]
    FWCorpKickMsg,
    #[serde(rename = "FWCorpLeaveMsg")]
    FWCorpLeaveMsg,
    #[serde(rename = "FWCorpWarningMsg")]
    FWCorpWarningMsg,
    #[serde(rename = "FacWarCorpJoinRequestMsg")]
    FacWarCorpJoinRequestMsg,
    #[serde(rename = "FacWarCorpJoinWithdrawMsg")]
    FacWarCorpJoinWithdrawMsg,
    #[serde(rename = "FacWarCorpLeaveRequestMsg")]
    FacWarCorpLeaveRequestMsg,
    #[serde(rename = "FacWarCorpLeaveWithdrawMsg")]
    FacWarCorpLeaveWithdrawMsg,
    #[serde(rename = "FacWarLPDisqualifiedEvent")]
    FacWarLPDisqualifiedEvent,
    #[serde(rename = "FacWarLPDisqualifiedKill")]
    FacWarLPDisqualifiedKill,
    #[serde(rename = "FacWarLPPayoutEvent")]
    FacWarLPPayoutEvent,
    #[serde(rename = "FacWarLPPayoutKill")]
    FacWarLPPayoutKill,
    #[serde(rename = "GameTimeAdded")]
    GameTimeAdded,
    #[serde(rename = "GameTimeReceived")]
    GameTimeReceived,
    #[serde(rename = "GameTimeSent")]
    GameTimeSent,
    #[serde(rename = "GiftReceived")]
    GiftReceived,
    #[serde(rename = "IHubDestroyedByBillFailure")]
    IHubDestroyedByBillFailure,
    #[serde(rename = "IncursionCompletedMsg")]
    IncursionCompletedMsg,
    #[serde(rename = "IndustryOperationFinished")]
    IndustryOperationFinished,
    #[serde(rename = "IndustryTeamAuctionLost")]
    IndustryTeamAuctionLost,
    #[serde(rename = "IndustryTeamAuctionWon")]
    IndustryTeamAuctionWon,
    #[serde(rename = "InfrastructureHubBillAboutToExpire")]
    InfrastructureHubBillAboutToExpire,
    #[serde(rename = "InsuranceExpirationMsg")]
    InsuranceExpirationMsg,
    #[serde(rename = "InsuranceFirstShipMsg")]
    InsuranceFirstShipMsg,
    #[serde(rename = "InsuranceInvalidatedMsg")]
    InsuranceInvalidatedMsg,
    #[serde(rename = "InsuranceIssuedMsg")]
    InsuranceIssuedMsg,
    #[serde(rename = "InsurancePayoutMsg")]
    InsurancePayoutMsg,
    #[serde(rename = "InvasionCompletedMsg")]
    InvasionCompletedMsg,
    #[serde(rename = "InvasionSystemLogin")]
    InvasionSystemLogin,
    #[serde(rename = "InvasionSystemStart")]
    InvasionSystemStart,
    #[serde(rename = "JumpCloneDeletedMsg1")]
    JumpCloneDeletedMsg1,
    #[serde(rename = "JumpCloneDeletedMsg2")]
    JumpCloneDeletedMsg2,
    #[serde(rename = "KillReportFinalBlow")]
    KillReportFinalBlow,
    #[serde(rename = "KillReportVictim")]
    KillReportVictim,
    #[serde(rename = "KillRightAvailable")]
    KillRightAvailable,
    #[serde(rename = "KillRightAvailableOpen")]
    KillRightAvailableOpen,
    #[serde(rename = "KillRightEarned")]
    KillRightEarned,
    #[serde(rename = "KillRightUnavailable")]
    KillRightUnavailable,
    #[serde(rename = "KillRightUnavailableOpen")]
    KillRightUnavailableOpen,
    #[serde(rename = "KillRightUsed")]
    KillRightUsed,
    #[serde(rename = "LocateCharMsg")]
    LocateCharMsg,
    #[serde(rename = "MadeWarMutual")]
    MadeWarMutual,
    #[serde(rename = "MercOfferRetractedMsg")]
    MercOfferRetractedMsg,
    #[serde(rename = "MercOfferedNegotiationMsg")]
    MercOfferedNegotiationMsg,
    #[serde(rename = "MissionCanceledTriglavian")]
    MissionCanceledTriglavian,
    #[serde(rename = "MissionOfferExpirationMsg")]
    MissionOfferExpirationMsg,
    #[serde(rename = "MissionTimeoutMsg")]
    MissionTimeoutMsg,
    #[serde(rename = "MoonminingAutomaticFracture")]
    MoonminingAutomaticFracture,
    #[serde(rename = "MoonminingExtractionCancelled")]
    MoonminingExtractionCancelled,
    #[serde(rename = "MoonminingExtractionFinished")]
    MoonminingExtractionFinished,
    #[serde(rename = "MoonminingExtractionStarted")]
    MoonminingExtractionStarted,
    #[serde(rename = "MoonminingLaserFired")]
    MoonminingLaserFired,
    #[serde(rename = "MutualWarExpired")]
    MutualWarExpired,
    #[serde(rename = "MutualWarInviteAccepted")]
    MutualWarInviteAccepted,
    #[serde(rename = "MutualWarInviteRejected")]
    MutualWarInviteRejected,
    #[serde(rename = "MutualWarInviteSent")]
    MutualWarInviteSent,
    #[serde(rename = "NPCStandingsGained")]
    NPCStandingsGained,
    #[serde(rename = "NPCStandingsLost")]
    NPCStandingsLost,
    #[serde(rename = "OfferToAllyRetracted")]
    OfferToAllyRetracted,
    #[serde(rename = "OfferedSurrender")]
    OfferedSurrender,
    #[serde(rename = "OfferedToAlly")]
    OfferedToAlly,
    #[serde(rename = "OfficeLeaseCanceledInsufficientStandings")]
    OfficeLeaseCanceledInsufficientStandings,
    #[serde(rename = "OldLscMessages")]
    OldLscMessages,
    #[serde(rename = "OperationFinished")]
    OperationFinished,
    #[serde(rename = "OrbitalAttacked")]
    OrbitalAttacked,
    #[serde(rename = "OrbitalReinforced")]
    OrbitalReinforced,
    #[serde(rename = "OwnershipTransferred")]
    OwnershipTransferred,
    #[serde(rename = "RaffleCreated")]
    RaffleCreated,
    #[serde(rename = "RaffleExpired")]
    RaffleExpired,
    #[serde(rename = "RaffleFinished")]
    RaffleFinished,
    #[serde(rename = "ReimbursementMsg")]
    ReimbursementMsg,
    #[serde(rename = "ResearchMissionAvailableMsg")]
    ResearchMissionAvailableMsg,
    #[serde(rename = "RetractsWar")]
    RetractsWar,
    #[serde(rename = "SeasonalChallengeCompleted")]
    SeasonalChallengeCompleted,
    #[serde(rename = "SovAllClaimAquiredMsg")]
    SovAllClaimAquiredMsg,
    #[serde(rename = "SovAllClaimLostMsg")]
    SovAllClaimLostMsg,
    #[serde(rename = "SovCommandNodeEventStarted")]
    SovCommandNodeEventStarted,
    #[serde(rename = "SovCorpBillLateMsg")]
    SovCorpBillLateMsg,
    #[serde(rename = "SovCorpClaimFailMsg")]
    SovCorpClaimFailMsg,
    #[serde(rename = "SovDisruptorMsg")]
    SovDisruptorMsg,
    #[serde(rename = "SovStationEnteredFreeport")]
    SovStationEnteredFreeport,
    #[serde(rename = "SovStructureDestroyed")]
    SovStructureDestroyed,
    #[serde(rename = "SovStructureReinforced")]
    SovStructureReinforced,
    #[serde(rename = "SovStructureSelfDestructCancel")]
    SovStructureSelfDestructCancel,
    #[serde(rename = "SovStructureSelfDestructFinished")]
    SovStructureSelfDestructFinished,
    #[serde(rename = "SovStructureSelfDestructRequested")]
    SovStructureSelfDestructRequested,
    #[serde(rename = "SovereigntyIHDamageMsg")]
    SovereigntyIHDamageMsg,
    #[serde(rename = "SovereigntySBUDamageMsg")]
    SovereigntySBUDamageMsg,
    #[serde(rename = "SovereigntyTCUDamageMsg")]
    SovereigntyTCUDamageMsg,
    #[serde(rename = "StationAggressionMsg1")]
    StationAggressionMsg1,
    #[serde(rename = "StationAggressionMsg2")]
    StationAggressionMsg2,
    #[serde(rename = "StationConquerMsg")]
    StationConquerMsg,
    #[serde(rename = "StationServiceDisabled")]
    StationServiceDisabled,
    #[serde(rename = "StationServiceEnabled")]
    StationServiceEnabled,
    #[serde(rename = "StationStateChangeMsg")]
    StationStateChangeMsg,
    #[serde(rename = "StoryLineMissionAvailableMsg")]
    StoryLineMissionAvailableMsg,
    #[serde(rename = "StructureAnchoring")]
    StructureAnchoring,
    #[serde(rename = "StructureCourierContractChanged")]
    StructureCourierContractChanged,
    #[serde(rename = "StructureDestroyed")]
    StructureDestroyed,
    #[serde(rename = "StructureFuelAlert")]
    StructureFuelAlert,
    #[serde(rename = "StructureImpendingAbandonmentAssetsAtRisk")]
    StructureImpendingAbandonmentAssetsAtRisk,
    #[serde(rename = "StructureItemsDelivered")]
    StructureItemsDelivered,
    #[serde(rename = "StructureItemsMovedToSafety")]
    StructureItemsMovedToSafety,
    #[serde(rename = "StructureLostArmor")]
    StructureLostArmor,
    #[serde(rename = "StructureLostShields")]
    StructureLostShields,
    #[serde(rename = "StructureOnline")]
    StructureOnline,
    #[serde(rename = "StructureServicesOffline")]
    StructureServicesOffline,
    #[serde(rename = "StructureUnanchoring")]
    StructureUnanchoring,
    #[serde(rename = "StructureUnderAttack")]
    StructureUnderAttack,
    #[serde(rename = "StructureWentHighPower")]
    StructureWentHighPower,
    #[serde(rename = "StructureWentLowPower")]
    StructureWentLowPower,
    #[serde(rename = "StructuresJobsCancelled")]
    StructuresJobsCancelled,
    #[serde(rename = "StructuresJobsPaused")]
    StructuresJobsPaused,
    #[serde(rename = "StructuresReinforcementChanged")]
    StructuresReinforcementChanged,
    #[serde(rename = "TowerAlertMsg")]
    TowerAlertMsg,
    #[serde(rename = "TowerResourceAlertMsg")]
    TowerResourceAlertMsg,
    #[serde(rename = "TransactionReversalMsg")]
    TransactionReversalMsg,
    #[serde(rename = "TutorialMsg")]
    TutorialMsg,
    #[serde(rename = "WarAdopted ")]
    WarAdopted,
    #[serde(rename = "WarAllyInherited")]
    WarAllyInherited,
    #[serde(rename = "WarAllyOfferDeclinedMsg")]
    WarAllyOfferDeclinedMsg,
    #[serde(rename = "WarConcordInvalidates")]
    WarConcordInvalidates,
    #[serde(rename = "WarDeclared")]
    WarDeclared,
    #[serde(rename = "WarEndedHqSecurityDrop")]
    WarEndedHqSecurityDrop,
    #[serde(rename = "WarHQRemovedFromSpace")]
    WarHQRemovedFromSpace,
    #[serde(rename = "WarInherited")]
    WarInherited,
    #[serde(rename = "WarInvalid")]
    WarInvalid,
    #[serde(rename = "WarRetracted")]
    WarRetracted,
    #[serde(rename = "WarRetractedByConcord")]
    WarRetractedByConcord,
    #[serde(rename = "WarSurrenderDeclinedMsg")]
    WarSurrenderDeclinedMsg,
    #[serde(rename = "WarSurrenderOfferMsg")]
    WarSurrenderOfferMsg,
}

impl Default for Type {
    fn default() -> Type {
        Self::AcceptedAlly
    }
}
