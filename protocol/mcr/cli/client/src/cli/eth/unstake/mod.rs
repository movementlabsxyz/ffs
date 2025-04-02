use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, U256};
use mcr_protocol_client_eth_core::config::Config;

#[derive(Parser)]
pub struct Unstake {
	#[clap(flatten)]
	config: Config,

	/// Amount to unstake
	#[clap(long)]
	amount: U256,
}

impl Unstake {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;
		client.unstake(self.amount).await?;
		println!("Successfully unstaked {} tokens", self.amount);
		Ok(())
	}
}
