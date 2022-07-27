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

/// struct for passing parameters to the method [`get_route_origin_destination`]
#[derive(Clone, Debug, Default)]
pub struct GetRouteOriginDestinationParams {
    /// destination solar system ID
    pub destination: i32,
    /// origin solar system ID
    pub origin: i32,
    /// avoid solar system ID(s)
    pub avoid: Option<Vec<i32>>,
    /// connected solar system pairs
    pub connections: Option<Vec<Vec<i32>>>,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// route security preference
    pub flag: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for typed successes of method [`get_route_origin_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRouteOriginDestinationSuccess {
    Status200(Vec<i32>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_route_origin_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRouteOriginDestinationError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetRouteOriginDestinationNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// Get the systems between origin and destination  --- Alternate route: `/dev/route/{origin}/{destination}/`  Alternate route: `/legacy/route/{origin}/{destination}/`  Alternate route: `/v1/route/{origin}/{destination}/`  --- This route is cached for up to 86400 seconds
pub async fn get_route_origin_destination(
    configuration: &configuration::Configuration,
    params: GetRouteOriginDestinationParams,
) -> Result<ResponseContent<GetRouteOriginDestinationSuccess>, Error<GetRouteOriginDestinationError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let destination = params.destination;
    let origin = params.origin;
    let avoid = params.avoid;
    let connections = params.connections;
    let datasource = params.datasource;
    let flag = params.flag;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/route/{origin}/{destination}/",
        local_var_configuration.base_path,
        destination = destination,
        origin = origin
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = avoid {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("avoid".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "avoid",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = connections {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| {
                        (
                            "connections".to_owned(),
                            p.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>()
                                .join(","),
                        )
                    })
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "connections",
                &local_var_str
                    .into_iter()
                    .map(|p| {
                        format!(
                            "[{}]",
                            p.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = flag {
        local_var_req_builder =
            local_var_req_builder.query(&[("flag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<GetRouteOriginDestinationSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetRouteOriginDestinationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
