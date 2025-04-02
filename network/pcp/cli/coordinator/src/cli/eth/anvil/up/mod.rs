use anyhow::Context;
use clap::Parser;
use jsonlvar::Jsonl;
use orfile::Orfile;
use pcp_network_anvil_component_core::dev::lifecycle::up::{Config, Up as UpCore};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Up {
	/// Config
	#[clap(flatten)]
	pub config: Config,
	/// Path to the configuration file
	#[clap(long)]
	pub write_artifacts_path: Option<String>,
	/// Path to write the anvil data as json
	#[clap(long)]
	pub write_anvil_data_path: Option<String>,
}

impl Up {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		println!("{}", config.try_to_jsonl_flat(None)?);

		let up = UpCore::new(config);
		let artifacts_state = up.artifacts().clone();
		let anvil_state = up.anvil_data().clone();

		let up_task = kestrel::task(async move { up.run().await });

		// print the anvil data and write it as a json to the specified path
		let anvil_data = anvil_state.read().wait_forever().await;
		println!("{}", anvil_data.try_to_jsonl_flat(None)?);
		if let Some(path) = &self.write_anvil_data_path {
			std::fs::write(path, serde_json::to_string(&anvil_data)?)
				.context("failed to write anvil data")?;
		}

		// print the artifacts and write it as a json to the specified path
		let artifacts = artifacts_state.read().wait_forever().await;
		println!("{}", artifacts.try_to_jsonl_flat(None)?);
		if let Some(path) = &self.write_artifacts_path {
			std::fs::write(path, serde_json::to_string(&artifacts)?)
				.context("failed to write artifacts")?;
		}

		up_task.await??;

		Ok(())
	}
}

impl or_file::Up {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
