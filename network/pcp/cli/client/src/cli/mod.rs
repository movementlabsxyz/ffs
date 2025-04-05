use clap::{Parser, Subcommand};

/// The `pcp-network-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpNetworkClient {
	#[clap(subcommand)]
	command: Option<PcpNetworkClientSubcommand>,
}

/// The subcommands of the `pcp-network-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum PcpNetworkClientSubcommand {
	/// A placeholder for future commands.
	Run,
}

/// Implement the `From` trait for `PcpNetworkClient` to convert it into a `PcpNetworkClientSubcommand`.
impl From<PcpNetworkClient> for PcpNetworkClientSubcommand {
	fn from(client: PcpNetworkClient) -> Self {
		client.command.unwrap_or(PcpNetworkClientSubcommand::Run)
	}
}

/// Implement the `PcpNetworkClient` CLI.
impl PcpNetworkClient {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpNetworkClientSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpNetworkClientSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpNetworkClientSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpNetworkClientSubcommand::Run => {
				println!("pcp-network-client is under development. Please check back later.");
			}
		}
		Ok(())
	}
}
