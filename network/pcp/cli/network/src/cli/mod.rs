use clap::{Parser, Subcommand};
use pcp_network_client::cli::PcpNetworkClientSubcommand;
use pcp_network_coordinator::cli::PcpNetworkCoordinatorSubcommand;

/// The `pcp-network` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpNetwork {
	#[clap(subcommand)]
	command: Option<PcpNetworkSubcommand>,
}

/// The subcommands of the `pcp-network` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum PcpNetworkSubcommand {
	/// A placeholder for future commands.
	Run,
	/// The client-specific commands of the PCP network.
	#[clap(subcommand)]
	Client(PcpNetworkClientSubcommand),
	/// The coordinator-specific commands of the PCP network.
	#[clap(subcommand)]
	Coordinator(PcpNetworkCoordinatorSubcommand),
}

/// Implement the `From` trait for `PcpNetwork` to convert it into a `PcpNetworkSubcommand`.
impl From<PcpNetwork> for PcpNetworkSubcommand {
	fn from(pcp_network: PcpNetwork) -> Self {
		pcp_network.command.unwrap_or(PcpNetworkSubcommand::Run)
	}
}

/// Implement the `PcpNetwork` CLI.
impl PcpNetwork {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpNetworkSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpNetworkSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpNetworkSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpNetworkSubcommand::Run => {
				println!("Welcome to the pcp-network CLI!");
			}
			PcpNetworkSubcommand::Client(client) => {
				client.execute().await?;
			}
			PcpNetworkSubcommand::Coordinator(coordinator) => {
				coordinator.execute().await?;
			}
		}
		Ok(())
	}
}
