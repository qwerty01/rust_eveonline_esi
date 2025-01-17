/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.17
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_alliances`]
#[derive(Clone, Debug, Default)]
pub struct GetAlliancesParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>
}

/// struct for passing parameters to the method [`get_alliances_alliance_id`]
#[derive(Clone, Debug, Default)]
pub struct GetAlliancesAllianceIdParams {
    /// An EVE alliance ID
    pub alliance_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>
}

/// struct for passing parameters to the method [`get_alliances_alliance_id_corporations`]
#[derive(Clone, Debug, Default)]
pub struct GetAlliancesAllianceIdCorporationsParams {
    /// An EVE alliance ID
    pub alliance_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>
}

/// struct for passing parameters to the method [`get_alliances_alliance_id_icons`]
#[derive(Clone, Debug, Default)]
pub struct GetAlliancesAllianceIdIconsParams {
    /// An EVE alliance ID
    pub alliance_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>
}


/// struct for typed successes of method [`get_alliances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesSuccess {
    Status200(Vec<i32>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_alliances_alliance_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdSuccess {
    Status200(crate::models::GetAlliancesAllianceIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_alliances_alliance_id_corporations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdCorporationsSuccess {
    Status200(Vec<i32>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_alliances_alliance_id_icons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdIconsSuccess {
    Status200(crate::models::GetAlliancesAllianceIdIconsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetAlliancesAllianceIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_corporations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdCorporationsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_alliances_alliance_id_icons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAlliancesAllianceIdIconsError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetAlliancesAllianceIdIconsNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// List all active player alliances  --- Alternate route: `/dev/alliances/`  Alternate route: `/legacy/alliances/`  Alternate route: `/v1/alliances/`  Alternate route: `/v2/alliances/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances(configuration: &configuration::Configuration, params: GetAlliancesParams) -> Result<ResponseContent<GetAlliancesSuccess>, Error<GetAlliancesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/alliances/", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetAlliancesSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAlliancesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Public information about an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/`  Alternate route: `/legacy/alliances/{alliance_id}/`  Alternate route: `/v3/alliances/{alliance_id}/`  Alternate route: `/v4/alliances/{alliance_id}/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances_alliance_id(configuration: &configuration::Configuration, params: GetAlliancesAllianceIdParams) -> Result<ResponseContent<GetAlliancesAllianceIdSuccess>, Error<GetAlliancesAllianceIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let alliance_id = params.alliance_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/alliances/{alliance_id}/", local_var_configuration.base_path, alliance_id=alliance_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetAlliancesAllianceIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAlliancesAllianceIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all current member corporations of an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/corporations/`  Alternate route: `/legacy/alliances/{alliance_id}/corporations/`  Alternate route: `/v1/alliances/{alliance_id}/corporations/`  Alternate route: `/v2/alliances/{alliance_id}/corporations/`  --- This route is cached for up to 3600 seconds
pub async fn get_alliances_alliance_id_corporations(configuration: &configuration::Configuration, params: GetAlliancesAllianceIdCorporationsParams) -> Result<ResponseContent<GetAlliancesAllianceIdCorporationsSuccess>, Error<GetAlliancesAllianceIdCorporationsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let alliance_id = params.alliance_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/alliances/{alliance_id}/corporations/", local_var_configuration.base_path, alliance_id=alliance_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetAlliancesAllianceIdCorporationsSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAlliancesAllianceIdCorporationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the icon urls for a alliance  --- Alternate route: `/legacy/alliances/{alliance_id}/icons/`  Alternate route: `/v1/alliances/{alliance_id}/icons/`  --- This route expires daily at 11:05  --- [Diff of the upcoming changes](https://esi.evetech.net/diff/latest/dev/#GET-/alliances/{alliance_id}/icons/)
pub async fn get_alliances_alliance_id_icons(configuration: &configuration::Configuration, params: GetAlliancesAllianceIdIconsParams) -> Result<ResponseContent<GetAlliancesAllianceIdIconsSuccess>, Error<GetAlliancesAllianceIdIconsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let alliance_id = params.alliance_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/alliances/{alliance_id}/icons/", local_var_configuration.base_path, alliance_id=alliance_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetAlliancesAllianceIdIconsSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetAlliancesAllianceIdIconsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

