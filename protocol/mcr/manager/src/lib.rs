mod manager;
pub use manager::Manager;

use mcr_protocol_client_core_util::McrClientError;
use mcr_types::commitment::{Commitment, CommitmentEvent};
use std::future::Future;
use tokio_stream::Stream;

pub type CommitmentEventStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<CommitmentEvent, McrClientError>> + Send>>;

pub trait McrManagerOperations {
	/// Adds a block commitment to the manager queue.
	fn post_commitment(
		&self,
		commitment: Commitment,
	) -> impl Future<Output = Result<(), McrClientError>>;
}
