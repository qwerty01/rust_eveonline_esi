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


/// struct for typed errors of method `get_characters_character_id_search`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSearchError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_search`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// Search for entities that match a given sub-string.  --- Alternate route: `/dev/characters/{character_id}/search/`  Alternate route: `/legacy/characters/{character_id}/search/`  Alternate route: `/v3/characters/{character_id}/search/`  --- This route is cached for up to 3600 seconds
pub async fn get_characters_character_id_search(configuration: &configuration::Configuration, categories: Vec<String>, character_id: i32, search: &str, accept_language: Option<&str>, datasource: Option<&str>, if_none_match: Option<&str>, language: Option<&str>, strict: Option<bool>, token: Option<&str>) -> Result<crate::models::GetCharactersCharacterIdSearchOk, Error<GetCharactersCharacterIdSearchError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/search/", configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("categories", &categories.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = language {
        local_var_req_builder = local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("search", &search.to_string())]);
    if let Some(ref local_var_str) = strict {
        local_var_req_builder = local_var_req_builder.query(&[("strict", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
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
        let local_var_entity: Option<GetCharactersCharacterIdSearchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for entities that match a given sub-string.  --- Alternate route: `/dev/search/`  Alternate route: `/legacy/search/`  Alternate route: `/v2/search/`  --- This route is cached for up to 3600 seconds
pub async fn get_search(configuration: &configuration::Configuration, categories: Vec<String>, search: &str, accept_language: Option<&str>, datasource: Option<&str>, if_none_match: Option<&str>, language: Option<&str>, strict: Option<bool>) -> Result<crate::models::GetSearchOk, Error<GetSearchError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/search/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("categories", &categories.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = language {
        local_var_req_builder = local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("search", &search.to_string())]);
    if let Some(ref local_var_str) = strict {
        local_var_req_builder = local_var_req_builder.query(&[("strict", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSearchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

