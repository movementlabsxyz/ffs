use mcr_types::block_commitment::BlockCommitment;
use std::future::Future;
use tokio_stream::Stream;

/// Errors thrown by the McrClient.
#[derive(Debug, thiserror::Error)]
pub enum McrClientError {
	#[error("failed to post block commitment: {0}")]
	PostBlockCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to invoke admin function: {0}")]
	AdminFunction(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to stream block commitments: {0}")]
	StreamBlockCommitments(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to get commitment: {0}")]
	GetCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type CommitmentStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<BlockCommitment, McrClientError>> + Send>>;

pub trait McrClientOperations {
	/// Posts a block commitment to the settlement client.
	fn post_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> impl Future<Output = Result<(), McrClientError>>;

	/// Posts a batch of block commitments to the settlement client.
	fn post_block_commitment_batch(
		&self,
		block_commitment: Vec<BlockCommitment>,
	) -> impl Future<Output = Result<(), McrClientError>>;

	/// Forces a block commitment
	/// This will only work in admin mode
	fn force_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> impl Future<Output = Result<(), McrClientError>>;

	/// Streams block commitments from the settlement client.
	fn stream_block_commitments(
		&self,
	) -> impl Future<Output = Result<CommitmentStream, McrClientError>>;

	/// Gets the accepted commitment at the given height.
	fn get_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<BlockCommitment>, McrClientError>>;

	/// Gets the commitment this validator has made at a given height
	fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<BlockCommitment>, McrClientError>>;

	/// Gets the max tolerable block height.
	fn get_max_tolerable_block_height(&self) -> impl Future<Output = Result<u64, McrClientError>>;
}
