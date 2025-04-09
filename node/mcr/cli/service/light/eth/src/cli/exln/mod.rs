use clap::Parser;
use mcr_light_node_eth_exln_core::config::Config as CoreConfig;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Runs the externalized Ethereum MCR light node.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct Exln {
	/// Path to the configuration file
	#[orfile(config)]
	#[clap(flatten)]
	pub core: CoreConfig,
}

impl Exln {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let core = self.core.clone();
		let node = core.build().await?;
		node.run().await?;

		Ok(())
	}
}

impl or_file::Exln {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
