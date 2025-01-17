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

/// struct for passing parameters to the method [`get_characters_character_id_calendar`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdCalendarParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// The event ID to retrieve events from
    pub from_event: Option<i32>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`get_characters_character_id_calendar_event_id`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdCalendarEventIdParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The id of the event requested
    pub event_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`get_characters_character_id_calendar_event_id_attendees`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdCalendarEventIdAttendeesParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The id of the event requested
    pub event_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`put_characters_character_id_calendar_event_id`]
#[derive(Clone, Debug, Default)]
pub struct PutCharactersCharacterIdCalendarEventIdParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The ID of the event requested
    pub event_id: i32,
    /// The response value to set, overriding current value
    pub response: crate::models::PutCharactersCharacterIdCalendarEventIdResponse,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}


/// struct for typed successes of method [`get_characters_character_id_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarSuccess {
    Status200(Vec<crate::models::GetCharactersCharacterIdCalendar200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_characters_character_id_calendar_event_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarEventIdSuccess {
    Status200(crate::models::GetCharactersCharacterIdCalendarEventIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_characters_character_id_calendar_event_id_attendees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarEventIdAttendeesSuccess {
    Status200(Vec<crate::models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`put_characters_character_id_calendar_event_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCharactersCharacterIdCalendarEventIdSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_calendar_event_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarEventIdError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status404(crate::models::GetCharactersCharacterIdCalendarEventIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_calendar_event_id_attendees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdCalendarEventIdAttendeesError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status404(crate::models::GetCharactersCharacterIdCalendarEventIdAttendeesNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_characters_character_id_calendar_event_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCharactersCharacterIdCalendarEventIdError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// Get 50 event summaries from the calendar. If no from_event ID is given, the resource will return the next 50 chronological event summaries from now. If a from_event ID is specified, it will return the next 50 chronological event summaries from after that event  --- Alternate route: `/dev/characters/{character_id}/calendar/`  Alternate route: `/legacy/characters/{character_id}/calendar/`  Alternate route: `/v1/characters/{character_id}/calendar/`  Alternate route: `/v2/characters/{character_id}/calendar/`  --- This route is cached for up to 5 seconds
pub async fn get_characters_character_id_calendar(configuration: &configuration::Configuration, params: GetCharactersCharacterIdCalendarParams) -> Result<ResponseContent<GetCharactersCharacterIdCalendarSuccess>, Error<GetCharactersCharacterIdCalendarError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let from_event = params.from_event;
    let if_none_match = params.if_none_match;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/calendar/", local_var_configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from_event {
        local_var_req_builder = local_var_req_builder.query(&[("from_event", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all the information for a specific event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v3/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v4/characters/{character_id}/calendar/{event_id}/`  --- This route is cached for up to 5 seconds
pub async fn get_characters_character_id_calendar_event_id(configuration: &configuration::Configuration, params: GetCharactersCharacterIdCalendarEventIdParams) -> Result<ResponseContent<GetCharactersCharacterIdCalendarEventIdSuccess>, Error<GetCharactersCharacterIdCalendarEventIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let event_id = params.event_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/calendar/{event_id}/", local_var_configuration.base_path, character_id=character_id, event_id=event_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarEventIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarEventIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all invited attendees for a given event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/v1/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/v2/characters/{character_id}/calendar/{event_id}/attendees/`  --- This route is cached for up to 600 seconds
pub async fn get_characters_character_id_calendar_event_id_attendees(configuration: &configuration::Configuration, params: GetCharactersCharacterIdCalendarEventIdAttendeesParams) -> Result<ResponseContent<GetCharactersCharacterIdCalendarEventIdAttendeesSuccess>, Error<GetCharactersCharacterIdCalendarEventIdAttendeesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let event_id = params.event_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/calendar/{event_id}/attendees/", local_var_configuration.base_path, character_id=character_id, event_id=event_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarEventIdAttendeesSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdCalendarEventIdAttendeesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set your response status to an event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v3/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v4/characters/{character_id}/calendar/{event_id}/`  --- This route is cached for up to 5 seconds
pub async fn put_characters_character_id_calendar_event_id(configuration: &configuration::Configuration, params: PutCharactersCharacterIdCalendarEventIdParams) -> Result<ResponseContent<PutCharactersCharacterIdCalendarEventIdSuccess>, Error<PutCharactersCharacterIdCalendarEventIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let event_id = params.event_id;
    let response = params.response;
    let datasource = params.datasource;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/calendar/{event_id}/", local_var_configuration.base_path, character_id=character_id, event_id=event_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&response);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PutCharactersCharacterIdCalendarEventIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PutCharactersCharacterIdCalendarEventIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

