use anyhow::Context;
use clap::Parser;
use pcp_dlu_eth_deployer_core::dev::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct Deploy {
	#[clap(flatten)]
	pub config: Option<Config>,
}

impl Deploy {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone().context("no config provided")?;
		let deployer = config.build()?;
		deployer.deploy().await?;
		Ok(())
	}
}
