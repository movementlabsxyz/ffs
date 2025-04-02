use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::block_commitment::{BlockCommitment, Commitment, Id};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct PostCommitment {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height of the block to commit
	#[clap(long)]
	height: u64,
	/// The id of the block to commit
	#[clap(long)]
	id: Id,
	/// The commitment to commit
	#[clap(long)]
	commitment: Commitment,
}

impl PostCommitment {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let block_commitment =
			BlockCommitment::new(self.height, self.id.clone(), self.commitment.clone());

		client.post_block_commitment(block_commitment).await?;

		Ok(())
	}
}

