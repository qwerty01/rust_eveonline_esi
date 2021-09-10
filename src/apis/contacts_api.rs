/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `delete_characters_character_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCharactersCharacterIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_alliances_alliance_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_alliances_alliance_id_contacts_labels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdContactsLabelsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_characters_character_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_characters_character_id_contacts_labels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdContactsLabelsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_corporations_corporation_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_corporations_corporation_id_contacts_labels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdContactsLabelsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_characters_character_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCharactersCharacterIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    Status520(crate::models::PostCharactersCharacterIdContactsError520),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `put_characters_character_id_contacts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCharactersCharacterIdContactsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// Bulk delete contacts  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn delete_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, contact_ids: Vec<i32>, datasource: Option<&str>, token: Option<&str>) -> Result<(), Error<DeleteCharactersCharacterIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("contact_ids", &contact_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteCharactersCharacterIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return contacts of an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/contacts/`  Alternate route: `/v2/alliances/{alliance_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_alliances_alliance_id_contacts(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<crate::models::GetAlliancesAllianceIdContacts200Ok>, Error<GetAlliancesAllianceIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/alliances/{alliance_id}/contacts/", configuration.base_path, alliance_id=alliance_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAlliancesAllianceIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return custom labels for an alliance's contacts  --- Alternate route: `/dev/alliances/{alliance_id}/contacts/labels/`  Alternate route: `/legacy/alliances/{alliance_id}/contacts/labels/`  Alternate route: `/v1/alliances/{alliance_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_alliances_alliance_id_contacts_labels(configuration: &configuration::Configuration, alliance_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<crate::models::GetAlliancesAllianceIdContactsLabels200Ok>, Error<GetAlliancesAllianceIdContactsLabelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/alliances/{alliance_id}/contacts/labels/", configuration.base_path, alliance_id=alliance_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAlliancesAllianceIdContactsLabelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return contacts of a character  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<crate::models::GetCharactersCharacterIdContacts200Ok>, Error<GetCharactersCharacterIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return custom labels for a character's contacts  --- Alternate route: `/dev/characters/{character_id}/contacts/labels/`  Alternate route: `/legacy/characters/{character_id}/contacts/labels/`  Alternate route: `/v1/characters/{character_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_characters_character_id_contacts_labels(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<crate::models::GetCharactersCharacterIdContactsLabels200Ok>, Error<GetCharactersCharacterIdContactsLabelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/contacts/labels/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdContactsLabelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return contacts of a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/contacts/`  Alternate route: `/v2/corporations/{corporation_id}/contacts/`  --- This route is cached for up to 300 seconds
pub async fn get_corporations_corporation_id_contacts(configuration: &configuration::Configuration, corporation_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, page: Option<i32>, token: Option<&str>) -> Result<Vec<crate::models::GetCorporationsCorporationIdContacts200Ok>, Error<GetCorporationsCorporationIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/corporations/{corporation_id}/contacts/", configuration.base_path, corporation_id=corporation_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCorporationsCorporationIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return custom labels for a corporation's contacts  --- Alternate route: `/dev/corporations/{corporation_id}/contacts/labels/`  Alternate route: `/legacy/corporations/{corporation_id}/contacts/labels/`  Alternate route: `/v1/corporations/{corporation_id}/contacts/labels/`  --- This route is cached for up to 300 seconds
pub async fn get_corporations_corporation_id_contacts_labels(configuration: &configuration::Configuration, corporation_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<crate::models::GetCorporationsCorporationIdContactsLabels200Ok>, Error<GetCorporationsCorporationIdContactsLabelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/corporations/{corporation_id}/contacts/labels/", configuration.base_path, corporation_id=corporation_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCorporationsCorporationIdContactsLabelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Bulk add contacts with same settings  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn post_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, standing: f32, contact_ids: Vec<i32>, datasource: Option<&str>, label_ids: Option<Vec<i64>>, token: Option<&str>, watched: Option<bool>) -> Result<Vec<i32>, Error<PostCharactersCharacterIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_ids {
        local_var_req_builder = local_var_req_builder.query(&[("label_ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("standing", &standing.to_string())]);
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = watched {
        local_var_req_builder = local_var_req_builder.query(&[("watched", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&contact_ids);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostCharactersCharacterIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Bulk edit contacts with same settings  --- Alternate route: `/dev/characters/{character_id}/contacts/`  Alternate route: `/v2/characters/{character_id}/contacts/` 
pub async fn put_characters_character_id_contacts(configuration: &configuration::Configuration, character_id: i32, standing: f32, contact_ids: Vec<i32>, datasource: Option<&str>, label_ids: Option<Vec<i64>>, token: Option<&str>, watched: Option<bool>) -> Result<(), Error<PutCharactersCharacterIdContactsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/contacts/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_ids {
        local_var_req_builder = local_var_req_builder.query(&[("label_ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("standing", &standing.to_string())]);
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = watched {
        local_var_req_builder = local_var_req_builder.query(&[("watched", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&contact_ids);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PutCharactersCharacterIdContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

