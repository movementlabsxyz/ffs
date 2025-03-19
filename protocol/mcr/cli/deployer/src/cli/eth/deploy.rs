use anyhow::Context;
use clap::Parser;
use mcr_protocol_deployer_core_eth::dev::config::Config;
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
		let artifacts = deployer.deploy().await?;
		println!("JSONL mcr_artifacts = {:?}", artifacts);
		Ok(())
	}
}
