pub use alloy_primitives::U256;
use mcr_types::commitment::Commitment;
use std::future::Future;
use tokio_stream::Stream;

/// Errors thrown by the McrClient.
#[derive(Debug, thiserror::Error)]
pub enum McrClientError {
	#[error("MCR Client failed to post block commitment: {0}")]
	PostCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("MCR client failed to invoke admin function: {0}")]
	AdminFunction(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("MCR client failed to stream block commitments: {0}")]
	StreamCommitments(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("MCR client failed to get commitment: {0}")]
	GetCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("MCR client encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Stream of block commitments from the settlement client.
pub type CommitmentStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<Commitment, McrClientError>> + Send>>;

pub trait McrClientOperations {
	/// Posts a block commitment to the settlement client.
	fn post_commitment(
		&self,
		commitment: Commitment,
	) -> impl Future<Output = Result<(), McrClientError>> + Send;

	/// Posts a batch of block commitments to the settlement client.
	fn post_commitment_batch(
		&self,
		commitment: Vec<Commitment>,
	) -> impl Future<Output = Result<(), McrClientError>> + Send;

	/// Forces a block commitment
	/// This will only work in admin mode
	fn force_commitment(
		&self,
		commitment: Commitment,
	) -> impl Future<Output = Result<(), McrClientError>> + Send;

	/// Streams block commitments from the settlement client.
	fn stream_commitments(
		&self,
	) -> impl Future<Output = Result<CommitmentStream, McrClientError>> + Send;

	/// Gets the accepted commitment at the given height.
	fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<Commitment>, McrClientError>> + Send;

	/// Gets the commitment this validator has made at a given height
	fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<Commitment>, McrClientError>> + Send;

	/// Gets the max tolerable block height.
	fn get_max_tolerable_commitment_height(
		&self,
	) -> impl Future<Output = Result<u64, McrClientError>> + Send;

	/// Gets the commitment for a specific validator at a given height
	fn get_validator_commitment_at_height(
		&self,
		height: u64,
		attester: String,
	) -> impl Future<Output = Result<Option<Commitment>, McrClientError>> + Send;

	/// Get the balance of the specified address
	fn get_balance(
		&self,
		address: String,
	) -> impl Future<Output = Result<u64, McrClientError>> + Send;

	/// Gets the last accepted block height.
	fn get_last_accepted_block_height(
		&self,
	) -> impl Future<Output = Result<u64, McrClientError>> + Send;

	/// Gets the leading block tolerance.
	fn get_leading_commitment_tolerance(
		&self,
	) -> impl Future<Output = Result<u64, McrClientError>> + Send;

	/// Grants TRUSTED_ATTESTER role to the specified address
	fn grant_trusted_attester(
		&self,
		attester: String,
	) -> impl Future<Output = Result<(), McrClientError>> + Send;

	/// Stakes tokens for the domain
	fn stake(&self, amount: U256) -> impl Future<Output = Result<(), McrClientError>> + Send;

	/// Get the current epoch stake for an attester
	fn get_stake(
		&self,
		custodian: String,
		attester: String,
	) -> impl Future<Output = Result<u64, McrClientError>> + Send;

	/// Unstakes tokens from the domain
	fn unstake(&self, amount: U256) -> impl Future<Output = Result<(), McrClientError>> + Send;
}

pub trait McrConfigOperations {
	type Client: McrClientOperations;

	fn build(&self) -> impl Future<Output = Result<Self::Client, McrClientError>> + Send;
}

pub trait McrViewOperations {
	type Config: McrConfigOperations;

	fn try_into_config(self) -> Result<Self::Config, McrClientError>;
}
