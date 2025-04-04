use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::block_commitment::{Commitment, CommitmentValue, Id};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct PostCommitmentBatch {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height of the block to commit
	#[clap(long)]
	height: u64,
	/// The id of the block to commit
	#[clap(long)]
	id: Id,
	/// The commitment value to commit
	#[clap(long)]
	commitment_value: CommitmentValue,
}

impl PostCommitmentBatch {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let block_commitment =
			Commitment::new(self.height, self.id.clone(), self.commitment_value.clone());
		let batch = vec![block_commitment];

		client.post_block_commitment_batch(batch).await?;

		Ok(())
	}
}
