pub mod eth;

use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

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
pub enum McrNetworkCoordinatorSubcommand {
	/// Generate markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Ethereum-specific commands of the network coordinator, i.e., for bringing-up an Ethereum-based MCR network.
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `McrNetworkCoordinator` to convert it into a `McrNetworkCoordinatorSubcommand`.
impl From<McrNetworkCoordinator> for McrNetworkCoordinatorSubcommand {
	fn from(coordinator: McrNetworkCoordinator) -> Self {
		coordinator
			.command
			.unwrap_or(McrNetworkCoordinatorSubcommand::Markdown(Markdown::default()))
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
			McrNetworkCoordinatorSubcommand::Markdown(markdown) => {
				markdown.execute::<McrNetworkCoordinator>().await?;
			}
			McrNetworkCoordinatorSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
