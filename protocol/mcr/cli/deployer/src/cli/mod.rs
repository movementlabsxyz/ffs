pub mod eth;
use clap::{Parser, Subcommand};

/// The `mcr-protocol-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrProtocolDeployer {
	#[clap(subcommand)]
	command: Option<McrProtocolDeployerSubcommand>,
}

/// The subcommands of the `mcr-protocol-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrProtocolDeployerSubcommand {
	/// ???
	Run,
	/// ???
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `McrProtocolDeployer` to convert it into a `McrProtocolDeployerSubcommand`.
impl From<McrProtocolDeployer> for McrProtocolDeployerSubcommand {
	fn from(client: McrProtocolDeployer) -> Self {
		client.command.unwrap_or(McrProtocolDeployerSubcommand::Run)
	}
}

/// Implement the `McrProtocolDeployer` CLI.
impl McrProtocolDeployer {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrProtocolDeployerSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrProtocolDeployerSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrProtocolDeployerSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrProtocolDeployerSubcommand::Run => {
				println!("mcr-protocol-client is under development. Please check back later.");
			}
			McrProtocolDeployerSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
