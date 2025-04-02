use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct GetCommitmentAtHeight {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height to get the commitment for
	#[clap(long)]
	height: u64,
}

impl GetCommitmentAtHeight {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		match client.get_accepted_commitment_at_height(self.height).await? {
			Some(commitment) => println!("Commitment at height {}: {:?}", self.height, commitment),
			None => println!("No commitment found at height {}", self.height),
		}

		Ok(())
	}
}
