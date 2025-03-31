pub mod eth;

use clap::{Parser, Subcommand};

/// The `mcr-network-coordinator` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrNetworkCoordinator {
	#[clap(subcommand)]
	command: Option<McrNetworkCoordinatorSubcommand>,
}

/// The subcommands of the `mcr-network-coordinator` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum McrNetworkCoordinatorSubcommand {
	/// ???
	Run,
	/// ???
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `McrNetworkCoordinator` to convert it into a `McrNetworkCoordinatorSubcommand`.
impl From<McrNetworkCoordinator> for McrNetworkCoordinatorSubcommand {
	fn from(coordinator: McrNetworkCoordinator) -> Self {
		coordinator.command.unwrap_or(McrNetworkCoordinatorSubcommand::Run)
	}
}

/// Implement the `McrNetworkCoordinator` CLI.
impl McrNetworkCoordinator {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrNetworkCoordinatorSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrNetworkCoordinatorSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrNetworkCoordinatorSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrNetworkCoordinatorSubcommand::Run => {
				println!("mcr-network-coordinator is under development. Please check back later.");
			}
			McrNetworkCoordinatorSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
