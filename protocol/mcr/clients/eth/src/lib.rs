use mcr_types::block_commitment::Commitment as BlockCommitment;
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

type CommitmentStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<BlockCommitment, anyhow::Error>> + Send>>;

#[async_trait::async_trait]
pub trait McrClientOperations {
	/// Posts a block commitment to the settlement client.
	async fn post_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> Result<(), McrClientError>;

	/// Posts a batch of block commitments to the settlement client.
	async fn post_block_commitment_batch(
		&self,
		block_commitment: Vec<BlockCommitment>,
	) -> Result<(), McrClientError>;

	/// Forces a block commitment
	/// This will only work in admin mode
	async fn force_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> Result<(), McrClientError>;

	/// Streams block commitments from the settlement client.
	async fn stream_block_commitments(&self) -> Result<CommitmentStream, McrClientError>;

	/// Gets the accepted commitment at the given height.
	async fn get_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<BlockCommitment>, McrClientError>;

	/// Gets the commitment this validator has made at a given height
	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<BlockCommitment>, McrClientError>;

	/// Gets the max tolerable block height.
	async fn get_max_tolerable_block_height(&self) -> Result<u64, McrClientError>;
}
