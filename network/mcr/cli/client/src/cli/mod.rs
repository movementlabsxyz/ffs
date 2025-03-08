use clap::{Parser, Subcommand};

/// The `mcr-network-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrNetworkClient {
	#[clap(subcommand)]
	command: Option<McrNetworkClientSubcommand>,
}

/// The subcommands of the `mcr-network-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrNetworkClientSubcommand {
	Run,
}

/// Implement the `From` trait for `McrNetworkClient` to convert it into a `McrNetworkClientSubcommand`.
impl From<McrNetworkClient> for McrNetworkClientSubcommand {
	fn from(client: McrNetworkClient) -> Self {
		client.command.unwrap_or(McrNetworkClientSubcommand::Run)
	}
}

/// Implement the `McrNetworkClient` CLI.
impl McrNetworkClient {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrNetworkClientSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrNetworkClientSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrNetworkClientSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrNetworkClientSubcommand::Run => {
				println!("mcr-network-client is under development. Please check back later.");
			}
		}
		Ok(())
	}
}
