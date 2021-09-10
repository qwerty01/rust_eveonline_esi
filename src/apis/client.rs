use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    alliance_api: Box<dyn crate::apis::AllianceApi>,
    assets_api: Box<dyn crate::apis::AssetsApi>,
    bookmarks_api: Box<dyn crate::apis::BookmarksApi>,
    calendar_api: Box<dyn crate::apis::CalendarApi>,
    character_api: Box<dyn crate::apis::CharacterApi>,
    clones_api: Box<dyn crate::apis::ClonesApi>,
    contacts_api: Box<dyn crate::apis::ContactsApi>,
    contracts_api: Box<dyn crate::apis::ContractsApi>,
    corporation_api: Box<dyn crate::apis::CorporationApi>,
    dogma_api: Box<dyn crate::apis::DogmaApi>,
    faction_warfare_api: Box<dyn crate::apis::FactionWarfareApi>,
    fittings_api: Box<dyn crate::apis::FittingsApi>,
    fleets_api: Box<dyn crate::apis::FleetsApi>,
    incursions_api: Box<dyn crate::apis::IncursionsApi>,
    industry_api: Box<dyn crate::apis::IndustryApi>,
    insurance_api: Box<dyn crate::apis::InsuranceApi>,
    killmails_api: Box<dyn crate::apis::KillmailsApi>,
    location_api: Box<dyn crate::apis::LocationApi>,
    loyalty_api: Box<dyn crate::apis::LoyaltyApi>,
    mail_api: Box<dyn crate::apis::MailApi>,
    market_api: Box<dyn crate::apis::MarketApi>,
    opportunities_api: Box<dyn crate::apis::OpportunitiesApi>,
    planetary_interaction_api: Box<dyn crate::apis::PlanetaryInteractionApi>,
    routes_api: Box<dyn crate::apis::RoutesApi>,
    search_api: Box<dyn crate::apis::SearchApi>,
    skills_api: Box<dyn crate::apis::SkillsApi>,
    sovereignty_api: Box<dyn crate::apis::SovereigntyApi>,
    status_api: Box<dyn crate::apis::StatusApi>,
    universe_api: Box<dyn crate::apis::UniverseApi>,
    user_interface_api: Box<dyn crate::apis::UserInterfaceApi>,
    wallet_api: Box<dyn crate::apis::WalletApi>,
    wars_api: Box<dyn crate::apis::WarsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            alliance_api: Box::new(crate::apis::AllianceApiClient::new(rc.clone())),
            assets_api: Box::new(crate::apis::AssetsApiClient::new(rc.clone())),
            bookmarks_api: Box::new(crate::apis::BookmarksApiClient::new(rc.clone())),
            calendar_api: Box::new(crate::apis::CalendarApiClient::new(rc.clone())),
            character_api: Box::new(crate::apis::CharacterApiClient::new(rc.clone())),
            clones_api: Box::new(crate::apis::ClonesApiClient::new(rc.clone())),
            contacts_api: Box::new(crate::apis::ContactsApiClient::new(rc.clone())),
            contracts_api: Box::new(crate::apis::ContractsApiClient::new(rc.clone())),
            corporation_api: Box::new(crate::apis::CorporationApiClient::new(rc.clone())),
            dogma_api: Box::new(crate::apis::DogmaApiClient::new(rc.clone())),
            faction_warfare_api: Box::new(crate::apis::FactionWarfareApiClient::new(rc.clone())),
            fittings_api: Box::new(crate::apis::FittingsApiClient::new(rc.clone())),
            fleets_api: Box::new(crate::apis::FleetsApiClient::new(rc.clone())),
            incursions_api: Box::new(crate::apis::IncursionsApiClient::new(rc.clone())),
            industry_api: Box::new(crate::apis::IndustryApiClient::new(rc.clone())),
            insurance_api: Box::new(crate::apis::InsuranceApiClient::new(rc.clone())),
            killmails_api: Box::new(crate::apis::KillmailsApiClient::new(rc.clone())),
            location_api: Box::new(crate::apis::LocationApiClient::new(rc.clone())),
            loyalty_api: Box::new(crate::apis::LoyaltyApiClient::new(rc.clone())),
            mail_api: Box::new(crate::apis::MailApiClient::new(rc.clone())),
            market_api: Box::new(crate::apis::MarketApiClient::new(rc.clone())),
            opportunities_api: Box::new(crate::apis::OpportunitiesApiClient::new(rc.clone())),
            planetary_interaction_api: Box::new(crate::apis::PlanetaryInteractionApiClient::new(rc.clone())),
            routes_api: Box::new(crate::apis::RoutesApiClient::new(rc.clone())),
            search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
            skills_api: Box::new(crate::apis::SkillsApiClient::new(rc.clone())),
            sovereignty_api: Box::new(crate::apis::SovereigntyApiClient::new(rc.clone())),
            status_api: Box::new(crate::apis::StatusApiClient::new(rc.clone())),
            universe_api: Box::new(crate::apis::UniverseApiClient::new(rc.clone())),
            user_interface_api: Box::new(crate::apis::UserInterfaceApiClient::new(rc.clone())),
            wallet_api: Box::new(crate::apis::WalletApiClient::new(rc.clone())),
            wars_api: Box::new(crate::apis::WarsApiClient::new(rc.clone())),
        }
    }

    pub fn alliance_api(&self) -> &dyn crate::apis::AllianceApi{
        self.alliance_api.as_ref()
    }

    pub fn assets_api(&self) -> &dyn crate::apis::AssetsApi{
        self.assets_api.as_ref()
    }

    pub fn bookmarks_api(&self) -> &dyn crate::apis::BookmarksApi{
        self.bookmarks_api.as_ref()
    }

    pub fn calendar_api(&self) -> &dyn crate::apis::CalendarApi{
        self.calendar_api.as_ref()
    }

    pub fn character_api(&self) -> &dyn crate::apis::CharacterApi{
        self.character_api.as_ref()
    }

    pub fn clones_api(&self) -> &dyn crate::apis::ClonesApi{
        self.clones_api.as_ref()
    }

    pub fn contacts_api(&self) -> &dyn crate::apis::ContactsApi{
        self.contacts_api.as_ref()
    }

    pub fn contracts_api(&self) -> &dyn crate::apis::ContractsApi{
        self.contracts_api.as_ref()
    }

    pub fn corporation_api(&self) -> &dyn crate::apis::CorporationApi{
        self.corporation_api.as_ref()
    }

    pub fn dogma_api(&self) -> &dyn crate::apis::DogmaApi{
        self.dogma_api.as_ref()
    }

    pub fn faction_warfare_api(&self) -> &dyn crate::apis::FactionWarfareApi{
        self.faction_warfare_api.as_ref()
    }

    pub fn fittings_api(&self) -> &dyn crate::apis::FittingsApi{
        self.fittings_api.as_ref()
    }

    pub fn fleets_api(&self) -> &dyn crate::apis::FleetsApi{
        self.fleets_api.as_ref()
    }

    pub fn incursions_api(&self) -> &dyn crate::apis::IncursionsApi{
        self.incursions_api.as_ref()
    }

    pub fn industry_api(&self) -> &dyn crate::apis::IndustryApi{
        self.industry_api.as_ref()
    }

    pub fn insurance_api(&self) -> &dyn crate::apis::InsuranceApi{
        self.insurance_api.as_ref()
    }

    pub fn killmails_api(&self) -> &dyn crate::apis::KillmailsApi{
        self.killmails_api.as_ref()
    }

    pub fn location_api(&self) -> &dyn crate::apis::LocationApi{
        self.location_api.as_ref()
    }

    pub fn loyalty_api(&self) -> &dyn crate::apis::LoyaltyApi{
        self.loyalty_api.as_ref()
    }

    pub fn mail_api(&self) -> &dyn crate::apis::MailApi{
        self.mail_api.as_ref()
    }

    pub fn market_api(&self) -> &dyn crate::apis::MarketApi{
        self.market_api.as_ref()
    }

    pub fn opportunities_api(&self) -> &dyn crate::apis::OpportunitiesApi{
        self.opportunities_api.as_ref()
    }

    pub fn planetary_interaction_api(&self) -> &dyn crate::apis::PlanetaryInteractionApi{
        self.planetary_interaction_api.as_ref()
    }

    pub fn routes_api(&self) -> &dyn crate::apis::RoutesApi{
        self.routes_api.as_ref()
    }

    pub fn search_api(&self) -> &dyn crate::apis::SearchApi{
        self.search_api.as_ref()
    }

    pub fn skills_api(&self) -> &dyn crate::apis::SkillsApi{
        self.skills_api.as_ref()
    }

    pub fn sovereignty_api(&self) -> &dyn crate::apis::SovereigntyApi{
        self.sovereignty_api.as_ref()
    }

    pub fn status_api(&self) -> &dyn crate::apis::StatusApi{
        self.status_api.as_ref()
    }

    pub fn universe_api(&self) -> &dyn crate::apis::UniverseApi{
        self.universe_api.as_ref()
    }

    pub fn user_interface_api(&self) -> &dyn crate::apis::UserInterfaceApi{
        self.user_interface_api.as_ref()
    }

    pub fn wallet_api(&self) -> &dyn crate::apis::WalletApi{
        self.wallet_api.as_ref()
    }

    pub fn wars_api(&self) -> &dyn crate::apis::WarsApi{
        self.wars_api.as_ref()
    }

}
