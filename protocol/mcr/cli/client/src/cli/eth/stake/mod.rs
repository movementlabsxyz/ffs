use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, U256};
use mcr_protocol_client_eth_core::config::Config;
#[derive(Parser)]
pub struct Stake {
	#[clap(flatten)]
	config: Config,

	/// Amount to stake
	#[clap(long)]
	amount: U256,
}

impl Stake {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;
		client.stake(self.amount).await?;
		println!("Successfully staked {} tokens", self.amount);
		Ok(())
	}
}
