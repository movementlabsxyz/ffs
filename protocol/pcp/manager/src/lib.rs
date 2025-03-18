use postconfirmations_types::block_commitment::{SuperBlockCommitment, SuperBlockCommitmentEvent};
use tokio_stream::Stream;

mod manager;

pub use manager::Manager as PcpSettlementManager;

pub type CommitmentEventStream =
	std::pin::Pin<Box<dyn Stream<Item = Result<SuperBlockCommitmentEvent, anyhow::Error>> + Send>>;

#[async_trait::async_trait]
pub trait PcpSettlementManagerOperations {
	/// Adds a block commitment to the manager queue.
	async fn post_block_commitment(
		&self,
		block_commitment: SuperBlockCommitment,
	) -> Result<(), anyhow::Error>;
}
