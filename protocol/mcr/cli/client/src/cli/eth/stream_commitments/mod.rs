use clap::Parser;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::Config;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct StreamCommitments {
	/// Config
	#[clap(flatten)]
	pub config: Config,
}

impl StreamCommitments {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		let mut stream = client.stream_block_commitments().await?;

		while let Some(result) = stream.next().await {
			match result {
				Ok(commitment) => println!("Received commitment: {:?}", commitment),
				Err(e) => eprintln!("Error receiving commitment: {:?}", e),
			}
		}

		Ok(())
	}
}
