use anyhow::Context;
use clap::Parser;
use jsonlvar::Jsonl;
use pcp_network_eth_live_component_core::dev::lifecycle::up::{Config, Up as UpCore};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct Up {
	/// Config
	#[clap(flatten)]
	pub config: Option<Config>,
}

impl Up {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone().context("no config provided")?;

		let up = UpCore::new(config);
		let artifacts_state = up.artifacts().clone();

		let up_task = kestrel::task(async move { up.run().await });

		let artifacts = artifacts_state.read().wait_forever().await;

		println!("{}", artifacts.try_to_jsonl_flat(None)?);

		up_task.await??;

		Ok(())
	}
}
