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

/// struct for passing parameters to the method [`get_characters_character_id_search`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdSearchParams {
    /// Type of entities to search for
    pub categories: Vec<String>,
    /// An EVE character ID
    pub character_id: i32,
    /// The string to search on
    pub search: String,
    /// Language to use in the response
    pub accept_language: Option<String>,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Language to use in the response, takes precedence over Accept-Language
    pub language: Option<String>,
    /// Whether the search should be a strict match
    pub strict: Option<bool>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`get_search`]
#[derive(Clone, Debug, Default)]
pub struct GetSearchParams {
    /// Type of entities to search for
    pub categories: Vec<String>,
    /// The string to search on
    pub search: String,
    /// Language to use in the response
    pub accept_language: Option<String>,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Language to use in the response, takes precedence over Accept-Language
    pub language: Option<String>,
    /// Whether the search should be a strict match
    pub strict: Option<bool>,
}

/// struct for typed successes of method [`get_characters_character_id_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdSearchSuccess {
    Status200(crate::models::GetCharactersCharacterIdSearchOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchSuccess {
    Status200(crate::models::GetSearchOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_search`]
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

/// struct for typed errors of method [`get_search`]
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
pub async fn get_characters_character_id_search(
    configuration: &configuration::Configuration,
    params: GetCharactersCharacterIdSearchParams,
) -> Result<
    ResponseContent<GetCharactersCharacterIdSearchSuccess>,
    Error<GetCharactersCharacterIdSearchError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let categories = params.categories;
    let character_id = params.character_id;
    let search = params.search;
    let accept_language = params.accept_language;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let language = params.language;
    let strict = params.strict;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{character_id}/search/",
        local_var_configuration.base_path,
        character_id = character_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[(
        "categories",
        &categories
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string(),
    )]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = language {
        local_var_req_builder =
            local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("search", &search.to_string())]);
    if let Some(ref local_var_str) = strict {
        local_var_req_builder =
            local_var_req_builder.query(&[("strict", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder =
            local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
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
        let local_var_entity: Option<GetCharactersCharacterIdSearchSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdSearchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Search for entities that match a given sub-string.  --- Alternate route: `/dev/search/`  Alternate route: `/legacy/search/`  Alternate route: `/v2/search/`  --- This route is cached for up to 3600 seconds
pub async fn get_search(
    configuration: &configuration::Configuration,
    params: GetSearchParams,
) -> Result<ResponseContent<GetSearchSuccess>, Error<GetSearchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let categories = params.categories;
    let search = params.search;
    let accept_language = params.accept_language;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let language = params.language;
    let strict = params.strict;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/search/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[(
        "categories",
        &categories
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string(),
    )]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = language {
        local_var_req_builder =
            local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("search", &search.to_string())]);
    if let Some(ref local_var_str) = strict {
        local_var_req_builder =
            local_var_req_builder.query(&[("strict", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder =
            local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder =
            local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetSearchSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetSearchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
