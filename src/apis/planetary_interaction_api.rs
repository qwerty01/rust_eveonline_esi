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

/// struct for passing parameters to the method [`get_characters_character_id_planets`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdPlanetsParams {
    /// An EVE character ID
    pub character_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`get_characters_character_id_planets_planet_id`]
#[derive(Clone, Debug, Default)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdParams {
    /// An EVE character ID
    pub character_id: i32,
    /// Planet id of the target planet
    pub planet_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`get_corporations_corporation_id_customs_offices`]
#[derive(Clone, Debug, Default)]
pub struct GetCorporationsCorporationIdCustomsOfficesParams {
    /// An EVE corporation ID
    pub corporation_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
    /// Which page of results to return
    pub page: Option<i32>,
    /// Access token to use if unable to set a header
    pub token: Option<String>
}

/// struct for passing parameters to the method [`get_universe_schematics_schematic_id`]
#[derive(Clone, Debug, Default)]
pub struct GetUniverseSchematicsSchematicIdParams {
    /// A PI schematic ID
    pub schematic_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>
}


/// struct for typed successes of method [`get_characters_character_id_planets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdPlanetsSuccess {
    Status200(Vec<crate::models::GetCharactersCharacterIdPlanets200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_characters_character_id_planets_planet_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdPlanetsPlanetIdSuccess {
    Status200(crate::models::GetCharactersCharacterIdPlanetsPlanetIdOk),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_corporations_corporation_id_customs_offices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdCustomsOfficesSuccess {
    Status200(Vec<crate::models::GetCorporationsCorporationIdCustomsOffices200Ok>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_universe_schematics_schematic_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUniverseSchematicsSchematicIdSuccess {
    Status200(crate::models::GetUniverseSchematicsSchematicIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_planets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdPlanetsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_planets_planet_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdPlanetsPlanetIdError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status404(crate::models::GetCharactersCharacterIdPlanetsPlanetIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_corporations_corporation_id_customs_offices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCorporationsCorporationIdCustomsOfficesError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_universe_schematics_schematic_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUniverseSchematicsSchematicIdError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetUniverseSchematicsSchematicIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// Returns a list of all planetary colonies owned by a character.  --- Alternate route: `/dev/characters/{character_id}/planets/`  Alternate route: `/legacy/characters/{character_id}/planets/`  Alternate route: `/v1/characters/{character_id}/planets/`  --- This route is cached for up to 600 seconds
pub async fn get_characters_character_id_planets(configuration: &configuration::Configuration, params: GetCharactersCharacterIdPlanetsParams) -> Result<ResponseContent<GetCharactersCharacterIdPlanetsSuccess>, Error<GetCharactersCharacterIdPlanetsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/planets/", local_var_configuration.base_path, character_id=character_id);
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
        let local_var_entity: Option<GetCharactersCharacterIdPlanetsSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdPlanetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns full details on the layout of a single planetary colony, including links, pins and routes. Note: Planetary information is only recalculated when the colony is viewed through the client. Information will not update until this criteria is met.  --- Alternate route: `/dev/characters/{character_id}/planets/{planet_id}/`  Alternate route: `/v3/characters/{character_id}/planets/{planet_id}/` 
pub async fn get_characters_character_id_planets_planet_id(configuration: &configuration::Configuration, params: GetCharactersCharacterIdPlanetsPlanetIdParams) -> Result<ResponseContent<GetCharactersCharacterIdPlanetsPlanetIdSuccess>, Error<GetCharactersCharacterIdPlanetsPlanetIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let character_id = params.character_id;
    let planet_id = params.planet_id;
    let datasource = params.datasource;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/planets/{planet_id}/", local_var_configuration.base_path, character_id=character_id, planet_id=planet_id);
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
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetCharactersCharacterIdPlanetsPlanetIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdPlanetsPlanetIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List customs offices owned by a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/customs_offices/`  Alternate route: `/legacy/corporations/{corporation_id}/customs_offices/`  Alternate route: `/v1/corporations/{corporation_id}/customs_offices/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 
pub async fn get_corporations_corporation_id_customs_offices(configuration: &configuration::Configuration, params: GetCorporationsCorporationIdCustomsOfficesParams) -> Result<ResponseContent<GetCorporationsCorporationIdCustomsOfficesSuccess>, Error<GetCorporationsCorporationIdCustomsOfficesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let corporation_id = params.corporation_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;
    let page = params.page;
    let token = params.token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/corporations/{corporation_id}/customs_offices/", local_var_configuration.base_path, corporation_id=corporation_id);
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
        let local_var_entity: Option<GetCorporationsCorporationIdCustomsOfficesSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCorporationsCorporationIdCustomsOfficesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information on a planetary factory schematic  --- Alternate route: `/dev/universe/schematics/{schematic_id}/`  Alternate route: `/legacy/universe/schematics/{schematic_id}/`  Alternate route: `/v1/universe/schematics/{schematic_id}/`  --- This route is cached for up to 3600 seconds
pub async fn get_universe_schematics_schematic_id(configuration: &configuration::Configuration, params: GetUniverseSchematicsSchematicIdParams) -> Result<ResponseContent<GetUniverseSchematicsSchematicIdSuccess>, Error<GetUniverseSchematicsSchematicIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let schematic_id = params.schematic_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/universe/schematics/{schematic_id}/", local_var_configuration.base_path, schematic_id=schematic_id);
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
        let local_var_entity: Option<GetUniverseSchematicsSchematicIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetUniverseSchematicsSchematicIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

