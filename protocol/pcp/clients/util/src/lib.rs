use pcp_types::commitment::Commitment;
use std::future::Future;
use tokio_stream::Stream;

/// Errors thrown by the PcpClient.
#[derive(Debug, thiserror::Error)]
pub enum PcpClientError {
	#[error("PCP Client failed to post block commitment: {0}")]
	PostCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("PCP client failed to invoke admin function: {0}")]
	AdminFunction(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("PCP client failed to stream block commitments: {0}")]
	StreamCommitments(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("PCP client failed to get commitment: {0}")]
	GetCommitment(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("PCP client encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Stream of block commitments from the settlement client.
pub type CommitmentStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<Commitment, PcpClientError>> + Send>>;

pub trait PcpClientOperations {
	/// Posts a block commitment to the settlement client.
	fn post_commitment(
		&self,
		commitment: Commitment,
	) -> impl Future<Output = Result<(), PcpClientError>> + Send;

	/// Posts a batch of block commitments to the settlement client.
	fn post_commitment_batch(
		&self,
		commitment: Vec<Commitment>,
	) -> impl Future<Output = Result<(), PcpClientError>> + Send;

	/// Forces a block commitment
	/// This will only work in admin mode
	fn force_commitment(
		&self,
		commitment: Commitment,
	) -> impl Future<Output = Result<(), PcpClientError>> + Send;

	/// Streams block commitments from the settlement client.
	fn stream_commitments(
		&self,
	) -> impl Future<Output = Result<CommitmentStream, PcpClientError>> + Send;

	/// Gets the accepted commitment at the given height.
	fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<Commitment>, PcpClientError>> + Send;

	/// Gets the commitment this validator has made at a given height
	fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> impl Future<Output = Result<Option<Commitment>, PcpClientError>> + Send;

	/// Gets the max tolerable block height.
	fn get_max_tolerable_commitment_height(
		&self,
	) -> impl Future<Output = Result<u64, PcpClientError>> + Send;
}
