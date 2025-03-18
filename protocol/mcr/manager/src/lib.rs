mod manager;
pub use manager::Manager;

use mcr_protocol_client_core_util::McrClientError;
use mcr_types::block_commitment::{BlockCommitment, BlockCommitmentEvent};
use std::future::Future;
use tokio_stream::Stream;

pub type CommitmentEventStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<BlockCommitmentEvent, McrClientError>> + Send>>;

pub trait McrManagerOperations {
	/// Adds a block commitment to the manager queue.
	fn post_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> impl Future<Output = Result<(), McrClientError>>;
}
