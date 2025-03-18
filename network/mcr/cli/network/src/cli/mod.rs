use clap::{Parser, Subcommand};
use mcr_network_client::cli::McrNetworkClientSubcommand;
use mcr_network_coordinator::cli::McrNetworkCoordinatorSubcommand;

/// The `mcr-network` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrNetwork {
	#[clap(subcommand)]
	command: Option<McrNetworkSubcommand>,
}

/// The subcommands of the `mcr-network` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrNetworkSubcommand {
	Run,
	#[clap(subcommand)]
	Client(McrNetworkClientSubcommand),
	#[clap(subcommand)]
	Coordinator(McrNetworkCoordinatorSubcommand),
}

/// Implement the `From` trait for `McrNetwork` to convert it into a `McrNetworkSubcommand`.
impl From<McrNetwork> for McrNetworkSubcommand {
	fn from(mcr_network: McrNetwork) -> Self {
		mcr_network.command.unwrap_or(McrNetworkSubcommand::Run)
	}
}

/// Implement the `McrNetwork` CLI.
impl McrNetwork {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrNetworkSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrNetworkSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrNetworkSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrNetworkSubcommand::Run => {
				println!("Welcome to the mcr-network CLI!");
			}
			McrNetworkSubcommand::Client(client) => {
				client.execute().await?;
			}
			McrNetworkSubcommand::Coordinator(coordinator) => {
				coordinator.execute().await?;
			}
		}
		Ok(())
	}
}
