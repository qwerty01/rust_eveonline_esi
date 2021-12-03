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

/// struct for passing parameters to the method [`get_dogma_attributes`]
#[derive(Clone, Debug, Default)]
pub struct GetDogmaAttributesParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_dogma_attributes_attribute_id`]
#[derive(Clone, Debug, Default)]
pub struct GetDogmaAttributesAttributeIdParams {
    /// A dogma attribute ID
    pub attribute_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_dogma_dynamic_items_type_id_item_id`]
#[derive(Clone, Debug, Default)]
pub struct GetDogmaDynamicItemsTypeIdItemIdParams {
    /// item_id integer
    pub item_id: i64,
    /// type_id integer
    pub type_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_dogma_effects`]
#[derive(Clone, Debug, Default)]
pub struct GetDogmaEffectsParams {
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for passing parameters to the method [`get_dogma_effects_effect_id`]
#[derive(Clone, Debug, Default)]
pub struct GetDogmaEffectsEffectIdParams {
    /// A dogma effect ID
    pub effect_id: i32,
    /// The server name you would like data from
    pub datasource: Option<String>,
    /// ETag from a previous request. A 304 will be returned if this matches the current ETag
    pub if_none_match: Option<String>,
}

/// struct for typed successes of method [`get_dogma_attributes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaAttributesSuccess {
    Status200(Vec<i32>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_dogma_attributes_attribute_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaAttributesAttributeIdSuccess {
    Status200(crate::models::GetDogmaAttributesAttributeIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_dogma_dynamic_items_type_id_item_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaDynamicItemsTypeIdItemIdSuccess {
    Status200(crate::models::GetDogmaDynamicItemsTypeIdItemIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_dogma_effects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaEffectsSuccess {
    Status200(Vec<i32>),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_dogma_effects_effect_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaEffectsEffectIdSuccess {
    Status200(crate::models::GetDogmaEffectsEffectIdOk),
    Status304(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dogma_attributes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaAttributesError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dogma_attributes_attribute_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaAttributesAttributeIdError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetDogmaAttributesAttributeIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dogma_dynamic_items_type_id_item_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaDynamicItemsTypeIdItemIdError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetDogmaDynamicItemsTypeIdItemIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dogma_effects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaEffectsError {
    Status400(crate::models::BadRequest),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dogma_effects_effect_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDogmaEffectsEffectIdError {
    Status400(crate::models::BadRequest),
    Status404(crate::models::GetDogmaEffectsEffectIdNotFound),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// Get a list of dogma attribute ids  --- Alternate route: `/dev/dogma/attributes/`  Alternate route: `/legacy/dogma/attributes/`  Alternate route: `/v1/dogma/attributes/`  --- This route expires daily at 11:05
pub async fn get_dogma_attributes(
    configuration: &configuration::Configuration,
    params: GetDogmaAttributesParams,
) -> Result<ResponseContent<GetDogmaAttributesSuccess>, Error<GetDogmaAttributesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/dogma/attributes/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDogmaAttributesSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDogmaAttributesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information on a dogma attribute  --- Alternate route: `/dev/dogma/attributes/{attribute_id}/`  Alternate route: `/legacy/dogma/attributes/{attribute_id}/`  Alternate route: `/v1/dogma/attributes/{attribute_id}/`  --- This route expires daily at 11:05
pub async fn get_dogma_attributes_attribute_id(
    configuration: &configuration::Configuration,
    params: GetDogmaAttributesAttributeIdParams,
) -> Result<
    ResponseContent<GetDogmaAttributesAttributeIdSuccess>,
    Error<GetDogmaAttributesAttributeIdError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let attribute_id = params.attribute_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/dogma/attributes/{attribute_id}/",
        local_var_configuration.base_path,
        attribute_id = attribute_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDogmaAttributesAttributeIdSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDogmaAttributesAttributeIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns info about a dynamic item resulting from mutation with a mutaplasmid.  --- Alternate route: `/dev/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/legacy/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/v1/dogma/dynamic/items/{type_id}/{item_id}/`  --- This route expires daily at 11:05
pub async fn get_dogma_dynamic_items_type_id_item_id(
    configuration: &configuration::Configuration,
    params: GetDogmaDynamicItemsTypeIdItemIdParams,
) -> Result<
    ResponseContent<GetDogmaDynamicItemsTypeIdItemIdSuccess>,
    Error<GetDogmaDynamicItemsTypeIdItemIdError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let item_id = params.item_id;
    let type_id = params.type_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/dogma/dynamic/items/{type_id}/{item_id}/",
        local_var_configuration.base_path,
        item_id = item_id,
        type_id = type_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDogmaDynamicItemsTypeIdItemIdSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDogmaDynamicItemsTypeIdItemIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of dogma effect ids  --- Alternate route: `/dev/dogma/effects/`  Alternate route: `/legacy/dogma/effects/`  Alternate route: `/v1/dogma/effects/`  --- This route expires daily at 11:05
pub async fn get_dogma_effects(
    configuration: &configuration::Configuration,
    params: GetDogmaEffectsParams,
) -> Result<ResponseContent<GetDogmaEffectsSuccess>, Error<GetDogmaEffectsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/dogma/effects/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDogmaEffectsSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDogmaEffectsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information on a dogma effect  --- Alternate route: `/dev/dogma/effects/{effect_id}/`  Alternate route: `/v2/dogma/effects/{effect_id}/`  --- This route expires daily at 11:05
pub async fn get_dogma_effects_effect_id(
    configuration: &configuration::Configuration,
    params: GetDogmaEffectsEffectIdParams,
) -> Result<ResponseContent<GetDogmaEffectsEffectIdSuccess>, Error<GetDogmaEffectsEffectIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let effect_id = params.effect_id;
    let datasource = params.datasource;
    let if_none_match = params.if_none_match;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/dogma/effects/{effect_id}/",
        local_var_configuration.base_path,
        effect_id = effect_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder =
            local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDogmaEffectsEffectIdSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetDogmaEffectsEffectIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
