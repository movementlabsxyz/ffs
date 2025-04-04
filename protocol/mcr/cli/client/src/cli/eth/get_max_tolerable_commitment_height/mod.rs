use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct GetMaxTolerableCommitmentHeight {
	/// Config
	#[clap(flatten)]
	pub config: Config,
}

impl GetMaxTolerableCommitmentHeight {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let height = client.get_max_tolerable_commitment_height().await?;
		println!("Max tolerable block height: {}", height);

		Ok(())
	}
}
