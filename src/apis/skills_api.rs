/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.10.1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`get_characters_character_id_attributes`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdAttributesParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`get_characters_character_id_skillqueue`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdSkillqueueParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`get_characters_character_id_skills`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdSkillsParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for typed successes of method [`get_characters_character_id_attributes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdAttributesSuccess {
    Status200(crate::models::GetCharactersCharacterIdAttributesOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_characters_character_id_skillqueue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSkillqueueSuccess {
    Status200(Vec<crate::models::GetCharactersCharacterIdSkillqueue200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_characters_character_id_skills`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSkillsSuccess {
    Status200(crate::models::GetCharactersCharacterIdSkillsOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_attributes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdAttributesError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_skillqueue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSkillqueueError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_skills`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSkillsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// Return attributes of a character  --- Alternate route: `/dev/characters/{character_id}/attributes/`  Alternate route: `/legacy/characters/{character_id}/attributes/`  Alternate route: `/v1/characters/{character_id}/attributes/`  --- This route is cached for up to 120 seconds
pub async fn get_characters_character_id_attributes(
    configuration: &configuration::Configuration,
    params: GetCharactersCharacterIdAttributesParams,
) -> Result<
    ResponseContent<GetCharactersCharacterIdAttributesSuccess>,
    Error<GetCharactersCharacterIdAttributesError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{character_id}/attributes/",
        local_var_configuration.base_path,
        character_id = character_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdAttributesSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdAttributesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the configured skill queue for the given character  --- Alternate route: `/dev/characters/{character_id}/skillqueue/`  Alternate route: `/legacy/characters/{character_id}/skillqueue/`  Alternate route: `/v2/characters/{character_id}/skillqueue/`  --- This route is cached for up to 120 seconds
pub async fn get_characters_character_id_skillqueue(
    configuration: &configuration::Configuration,
    params: GetCharactersCharacterIdSkillqueueParams,
) -> Result<
    ResponseContent<GetCharactersCharacterIdSkillqueueSuccess>,
    Error<GetCharactersCharacterIdSkillqueueError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{character_id}/skillqueue/",
        local_var_configuration.base_path,
        character_id = character_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdSkillqueueSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdSkillqueueError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all trained skills for the given character  --- Alternate route: `/dev/characters/{character_id}/skills/`  Alternate route: `/v4/characters/{character_id}/skills/`  --- This route is cached for up to 120 seconds
pub async fn get_characters_character_id_skills(
    configuration: &configuration::Configuration,
    params: GetCharactersCharacterIdSkillsParams,
) -> Result<
    ResponseContent<GetCharactersCharacterIdSkillsSuccess>,
    Error<GetCharactersCharacterIdSkillsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{character_id}/skills/",
        local_var_configuration.base_path,
        character_id = character_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdSkillsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdSkillsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
