use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::commitment::{Commitment, Id, Vote};
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
	vote: Vote,
}

impl PostCommitmentBatch {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let commitment = Commitment::new(self.height, self.id.clone(), self.vote.clone());
		let batch = vec![commitment];

		client.post_commitment_batch(batch).await?;

		Ok(())
	}
}
