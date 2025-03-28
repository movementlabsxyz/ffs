use clap::Parser;
use jsonlvar::Jsonl;
use mcr_protocol_deployer_eth_core::Lifecycle;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct Deploy {
	#[clap(flatten)]
	pub config: Config,
}

impl Deploy {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let artifacts = self.config.apply().await?;
		println!("{}", artifacts.try_to_jsonl_flat(None)?);
		Ok(())
	}
}
