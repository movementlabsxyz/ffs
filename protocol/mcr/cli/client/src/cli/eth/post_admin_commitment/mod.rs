use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use mcr_types::commitment::{Commitment, Id, Vote};
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
	id: Id,
	/// The commitment value to commit
	#[clap(long)]
	vote: Vote,
}

impl PostAdminCommitment {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let height = match self.height {
			Some(height) => height,
			None => client.get_max_tolerable_commitment_height().await?,
		};

		let commitment = Commitment::new(height, self.id.clone(), self.vote.clone());

		client.force_commitment(commitment).await?;

		Ok(())
	}
}
