use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::block_commitment::{Commitment, CommitmentValue, Id};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct PostAdminCommitment {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height of the commitment block at which to commit
	#[clap(long)]
	height: Option<u64>,
	/// The id of the commitment block at which to commit
	#[clap(long)]
	id: Id,
	/// The commitment value to commit
	#[clap(long)]
	commitment_value: CommitmentValue,
}

impl PostAdminCommitment {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let height = match self.height {
			Some(height) => height,
			None => client.get_max_tolerable_block_height().await?,
		};

		let block_commitment =
			Commitment::new(height, self.id.clone(), self.commitment_value.clone());

		client.force_block_commitment(block_commitment).await?;

		Ok(())
	}
}
