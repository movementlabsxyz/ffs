use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::commitment::{Commitment, CommitmentValue, CommitmentId};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct PostCommitment {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height of the commitment block at which to commit
	#[clap(long)]
	height: u64,
	/// The id of the commitment block at which to commit
	#[clap(long)]
	commitment_id: CommitmentId,
	/// The commitment value to commit
	#[clap(long)]
	commitment_value: CommitmentValue,
}

impl PostCommitment {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let commitment =
			Commitment::new(self.height, self.commitment_id.clone(), self.commitment_value.clone());

		client.post_commitment(commitment).await?;

		Ok(())
	}
}

