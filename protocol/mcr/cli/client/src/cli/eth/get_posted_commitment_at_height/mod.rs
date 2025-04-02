use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct GetPostedCommitmentAtHeight {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// The height to get the commitment for
	#[clap(long)]
	height: u64,
}

impl GetPostedCommitmentAtHeight {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		match client.get_posted_commitment_at_height(self.height).await? {
			Some(commitment) => {
				println!("Posted commitment at height {}: {:?}", self.height, commitment)
			}
			None => println!("No posted commitment found at height {}", self.height),
		}

		Ok(())
	}
}
