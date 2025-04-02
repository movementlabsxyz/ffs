use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactsError {
	#[error("JSON parsing error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Missing or invalid field: {0}")]
	MissingField(String),
}

/// The artifacts produced by the PCP dev deployment.
#[derive(Debug, Clone, Serialize, Deserialize, Jsonl)]
pub struct Artifacts {
	pub proxy_admin: String,

	// Implementations
	pub move_token_implementation: String,
	pub staking_implementation: String,
	pub pcp_implementation: String,

	// Proxies
	pub token_proxy: String,
	pub movement_staking_proxy: String,
	pub pcp_proxy: String,

	// Commitment Admin Grants
	pub granted_commitment_admin: String,

	// Custodian Setup
	pub pcp_custodian_epoch_duration: u64,

	// Minter Roles
	pub granted_minter_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintedTokens {
	pub recipient: String,
	pub amount: String,
}
