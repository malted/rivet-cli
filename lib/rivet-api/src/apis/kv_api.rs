/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`kv_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvDeleteError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_delete_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvDeleteBatchError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvGetError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_get_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvGetBatchError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvListError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvPutError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kv_put_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KvPutBatchError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// Deletes a key-value entry by key.
pub async fn kv_delete(
	configuration: &configuration::Configuration,
	key: &str,
	namespace_id: Option<&str>,
) -> Result<(), Error<KvDeleteError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
	if let Some(ref local_var_str) = namespace_id {
		local_var_req_builder =
			local_var_req_builder.query(&[("namespace_id", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<KvDeleteError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Deletes multiple key-value entries by key(s).
pub async fn kv_delete_batch(
	configuration: &configuration::Configuration,
	keys: &str,
	namespace_id: Option<&str>,
) -> Result<(), Error<KvDeleteBatchError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries/batch", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("keys", &keys.to_string())]);
	if let Some(ref local_var_str) = namespace_id {
		local_var_req_builder =
			local_var_req_builder.query(&[("namespace_id", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<KvDeleteBatchError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Returns a specific key-value entry by key.
pub async fn kv_get(
	configuration: &configuration::Configuration,
	key: &str,
	watch_index: Option<&str>,
	namespace_id: Option<&str>,
) -> Result<crate::models::KvGetResponse, Error<KvGetError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_str) = namespace_id {
		local_var_req_builder =
			local_var_req_builder.query(&[("namespace_id", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<KvGetError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Gets multiple key-value entries by key(s).
pub async fn kv_get_batch(
	configuration: &configuration::Configuration,
	keys: &str,
	watch_index: Option<&str>,
	namespace_id: Option<&str>,
) -> Result<crate::models::KvGetBatchResponse, Error<KvGetBatchError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries/batch", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("keys", &keys.to_string())]);
	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_str) = namespace_id {
		local_var_req_builder =
			local_var_req_builder.query(&[("namespace_id", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<KvGetBatchError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Lists all keys in a directory.
pub async fn kv_list(
	configuration: &configuration::Configuration,
	directory: &str,
	namespace_id: &str,
) -> Result<crate::models::KvListResponse, Error<KvListError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries/list", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	local_var_req_builder = local_var_req_builder.query(&[("directory", &directory.to_string())]);
	local_var_req_builder =
		local_var_req_builder.query(&[("namespace_id", &namespace_id.to_string())]);
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<KvListError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Puts (sets or overwrites) a key-value entry by key.
pub async fn kv_put(
	configuration: &configuration::Configuration,
	kv_put_request: crate::models::KvPutRequest,
) -> Result<(), Error<KvPutError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&kv_put_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<KvPutError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Puts (sets or overwrites) multiple key-value entries by key(s).
pub async fn kv_put_batch(
	configuration: &configuration::Configuration,
	kv_put_batch_request: crate::models::KvPutBatchRequest,
) -> Result<(), Error<KvPutBatchError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/kv/entries/batch", local_var_configuration.base_path);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&kv_put_batch_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<KvPutBatchError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}
