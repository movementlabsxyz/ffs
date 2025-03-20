pub mod eth;
use clap::{Parser, Subcommand};

/// The `pcp-protocol-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpProtocolDeployer {
	#[clap(subcommand)]
	command: Option<PcpProtocolDeployerSubcommand>,
}

/// The subcommands of the `pcp-protocol-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum PcpProtocolDeployerSubcommand {
	Run,
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `PcpProtocolDeployer` to convert it into a `PcpProtocolDeployerSubcommand`.
impl From<PcpProtocolDeployer> for PcpProtocolDeployerSubcommand {
	fn from(client: PcpProtocolDeployer) -> Self {
		client.command.unwrap_or(PcpProtocolDeployerSubcommand::Run)
	}
}

/// Implement the `PcpProtocolDeployer` CLI.
impl PcpProtocolDeployer {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpProtocolDeployerSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpProtocolDeployerSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpProtocolDeployerSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpProtocolDeployerSubcommand::Run => {
				println!("pcp-protocol-client is under development. Please check back later.");
			}
			PcpProtocolDeployerSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
