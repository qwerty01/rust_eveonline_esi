/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`post_ui_autopilot_waypoint`]
#[derive(Clone, Debug, Default)]
pub struct PostUiAutopilotWaypointParams {
    /// Whether this solar system should be added to the beginning of all waypoints
    pub add_to_beginning: bool,
    /// Whether clean other waypoints beforing adding this one
    pub clear_other_waypoints: bool,
    /// The destination to travel to, can be solar system, station or structure's id
    pub destination_id: i64,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`post_ui_openwindow_contract`]
#[derive(Clone, Debug, Default)]
pub struct PostUiOpenwindowContractParams {
    /// The contract to open
    pub contract_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`post_ui_openwindow_information`]
#[derive(Clone, Debug, Default)]
pub struct PostUiOpenwindowInformationParams {
    /// The target to open
    pub target_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`post_ui_openwindow_marketdetails`]
#[derive(Clone, Debug, Default)]
pub struct PostUiOpenwindowMarketdetailsParams {
    /// The item type to open in market window
    pub type_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for passing parameters to the method [`post_ui_openwindow_newmail`]
#[derive(Clone, Debug, Default)]
pub struct PostUiOpenwindowNewmailParams {
    /// The details of mail to create
    pub new_mail: crate::models::PostUiOpenwindowNewmailNewMail,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// Access token to use if unable to set a header
    pub token: Option<String>,
}

/// struct for typed successes of method [`post_ui_autopilot_waypoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiAutopilotWaypointSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ui_openwindow_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowContractSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ui_openwindow_information`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowInformationSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ui_openwindow_marketdetails`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowMarketdetailsSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`post_ui_openwindow_newmail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowNewmailSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ui_autopilot_waypoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiAutopilotWaypointError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ui_openwindow_contract`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowContractError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ui_openwindow_information`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowInformationError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ui_openwindow_marketdetails`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowMarketdetailsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_ui_openwindow_newmail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUiOpenwindowNewmailError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status422(crate::models::PostUiOpenwindowNewmailUnprocessableEntity),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// Set a solar system as autopilot waypoint  --- Alternate route: `/dev/ui/autopilot/waypoint/`  Alternate route: `/legacy/ui/autopilot/waypoint/`  Alternate route: `/v2/ui/autopilot/waypoint/`
pub async fn post_ui_autopilot_waypoint(
    configuration: &configuration::Configuration,
    params: PostUiAutopilotWaypointParams,
) -> Result<ResponseContent<PostUiAutopilotWaypointSuccess>, Error<PostUiAutopilotWaypointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let add_to_beginning = params.add_to_beginning;
    let clear_other_waypoints = params.clear_other_waypoints;
    let destination_id = params.destination_id;
    let datasource = params.datasource;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/ui/autopilot/waypoint/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder =
        local_var_req_builder.query(&[("add_to_beginning", &add_to_beginning.to_string())]);
    local_var_req_builder = local_var_req_builder
        .query(&[("clear_other_waypoints", &clear_other_waypoints.to_string())]);
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    local_var_req_builder =
        local_var_req_builder.query(&[("destination_id", &destination_id.to_string())]);
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostUiAutopilotWaypointSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostUiAutopilotWaypointError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Open the contract window inside the client  --- Alternate route: `/dev/ui/openwindow/contract/`  Alternate route: `/legacy/ui/openwindow/contract/`  Alternate route: `/v1/ui/openwindow/contract/`
pub async fn post_ui_openwindow_contract(
    configuration: &configuration::Configuration,
    params: PostUiOpenwindowContractParams,
) -> Result<ResponseContent<PostUiOpenwindowContractSuccess>, Error<PostUiOpenwindowContractError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let contract_id = params.contract_id;
    let datasource = params.datasource;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/ui/openwindow/contract/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder =
        local_var_req_builder.query(&[("contract_id", &contract_id.to_string())]);
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
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostUiOpenwindowContractSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostUiOpenwindowContractError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Open the information window for a character, corporation or alliance inside the client  --- Alternate route: `/dev/ui/openwindow/information/`  Alternate route: `/legacy/ui/openwindow/information/`  Alternate route: `/v1/ui/openwindow/information/`
pub async fn post_ui_openwindow_information(
    configuration: &configuration::Configuration,
    params: PostUiOpenwindowInformationParams,
) -> Result<
    ResponseContent<PostUiOpenwindowInformationSuccess>,
    Error<PostUiOpenwindowInformationError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let target_id = params.target_id;
    let datasource = params.datasource;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/ui/openwindow/information/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("target_id", &target_id.to_string())]);
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostUiOpenwindowInformationSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostUiOpenwindowInformationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Open the market details window for a specific typeID inside the client  --- Alternate route: `/dev/ui/openwindow/marketdetails/`  Alternate route: `/legacy/ui/openwindow/marketdetails/`  Alternate route: `/v1/ui/openwindow/marketdetails/`
pub async fn post_ui_openwindow_marketdetails(
    configuration: &configuration::Configuration,
    params: PostUiOpenwindowMarketdetailsParams,
) -> Result<
    ResponseContent<PostUiOpenwindowMarketdetailsSuccess>,
    Error<PostUiOpenwindowMarketdetailsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let type_id = params.type_id;
    let datasource = params.datasource;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/ui/openwindow/marketdetails/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder =
            local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("type_id", &type_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostUiOpenwindowMarketdetailsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostUiOpenwindowMarketdetailsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Open the New Mail window, according to settings from the request if applicable  --- Alternate route: `/dev/ui/openwindow/newmail/`  Alternate route: `/legacy/ui/openwindow/newmail/`  Alternate route: `/v1/ui/openwindow/newmail/`
pub async fn post_ui_openwindow_newmail(
    configuration: &configuration::Configuration,
    params: PostUiOpenwindowNewmailParams,
) -> Result<ResponseContent<PostUiOpenwindowNewmailSuccess>, Error<PostUiOpenwindowNewmailError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let new_mail = params.new_mail;
    let datasource = params.datasource;
    let token = params.token;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/ui/openwindow/newmail/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&new_mail);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<PostUiOpenwindowNewmailSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<PostUiOpenwindowNewmailError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
