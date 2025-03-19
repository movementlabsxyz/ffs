use jsonlvar::JsonlParser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactsError {
	#[error("JSON parsing error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Missing or invalid field: {0}")]
	MissingField(String),
}

/// The artifacts produced by the MCR dev deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifacts {
	pub proxy_admin: String,

	// Implementations
	pub move_token_implementation: String,
	pub staking_implementation: String,
	pub mcr_implementation: String,

	// Proxies
	pub move_token_proxy: String,
	pub movement_staking_proxy: String,
	pub mcr_proxy: String,

	// Commitment Admin Grants
	pub granted_commitment_admin: String,

	// Custodian Setup
	pub mcr_custodian_epoch_duration: u64,

	// Minter Roles
	pub granted_minter_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintedTokens {
	pub recipient: String,
	pub amount: String,
}

impl Artifacts {
	pub fn try_from_jsonl(jsonl: &str) -> Result<Self, ArtifactsError> {
		let parser = JsonlParser::new();
		let parsed_data = parser.parse(jsonl);

		// Helper function to extract required string values or return an error
		fn extract_string(
			data: &HashMap<String, Value>,
			key: &str,
		) -> Result<String, ArtifactsError> {
			data.get(key)
				.and_then(Value::as_str)
				.map(String::from)
				.ok_or_else(|| ArtifactsError::MissingField(key.to_string()))
		}

		// Helper function to extract required u64 values or return an error
		fn extract_u64(data: &HashMap<String, Value>, key: &str) -> Result<u64, ArtifactsError> {
			data.get(key)
				.and_then(Value::as_u64)
				.ok_or_else(|| ArtifactsError::MissingField(key.to_string()))
		}

		// Helper function to extract required array of strings
		fn extract_string_array(
			data: &HashMap<String, Value>,
			key: &str,
		) -> Result<Vec<String>, ArtifactsError> {
			data.get(key)
				.and_then(Value::as_array)
				.map(|arr| arr.iter().filter_map(Value::as_str).map(String::from).collect())
				.ok_or_else(|| ArtifactsError::MissingField(key.to_string()))
		}

		Ok(Self {
			proxy_admin: extract_string(&parsed_data, "proxy_admin")?,
			move_token_implementation: extract_string(&parsed_data, "move_token_implementation")?,
			staking_implementation: extract_string(&parsed_data, "staking_implementation")?,
			mcr_implementation: extract_string(&parsed_data, "mcr_implementation")?,
			move_token_proxy: extract_string(&parsed_data, "move_token_proxy")?,
			movement_staking_proxy: extract_string(&parsed_data, "movement_staking_proxy")?,
			mcr_proxy: extract_string(&parsed_data, "mcr_proxy")?,
			granted_commitment_admin: extract_string(&parsed_data, "granted_commitment_admin")?,
			mcr_custodian_epoch_duration: extract_u64(
				&parsed_data,
				"mcr_custodian_epoch_duration",
			)?,
			granted_minter_role: extract_string(&parsed_data, "granted_minter_role")?,
		})
	}
}
