use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::commitment::{Commitment, CommitmentValue, CommitmentId};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct PostAdminCommitment {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The commitment height at which to commit
	#[clap(long)]
	height: Option<u64>,
	/// The commitment id to commit
	#[clap(long)]
	commitment_id: CommitmentId,
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
			None => client.get_max_tolerable_commitment_height().await?,
		};

		let commitment =
			Commitment::new(height, self.commitment_id.clone(), self.commitment_value.clone());

		client.force_commitment(commitment).await?;

		Ok(())
	}
}
