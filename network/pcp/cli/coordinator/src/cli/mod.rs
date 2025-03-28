pub mod eth;

use clap::{Parser, Subcommand};

/// The `pcp-network-coordinator` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpNetworkCoordinator {
	#[clap(subcommand)]
	command: Option<PcpNetworkCoordinatorSubcommand>,
}

/// The subcommands of the `pcp-network-coordinator` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum PcpNetworkCoordinatorSubcommand {
	/// ???
	Run,
	/// ???
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `PcpNetworkCoordinator` to convert it into a `PcpNetworkCoordinatorSubcommand`.
impl From<PcpNetworkCoordinator> for PcpNetworkCoordinatorSubcommand {
	fn from(coordinator: PcpNetworkCoordinator) -> Self {
		coordinator.command.unwrap_or(PcpNetworkCoordinatorSubcommand::Run)
	}
}

/// Implement the `PcpNetworkCoordinator` CLI.
impl PcpNetworkCoordinator {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpNetworkCoordinatorSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpNetworkCoordinatorSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpNetworkCoordinatorSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpNetworkCoordinatorSubcommand::Run => {
				println!("pcp-network-coordinator is under development. Please check back later.");
			}
			PcpNetworkCoordinatorSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
