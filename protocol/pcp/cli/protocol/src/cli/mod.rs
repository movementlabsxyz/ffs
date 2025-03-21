use clap::{Parser, Subcommand};
use pcp_protocol_client::cli::PcpProtocolClientSubcommand;

/// The `pcp-protocol` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpProtocol {
	#[clap(subcommand)]
	command: Option<PcpProtocolSubcommand>,
}

/// The subcommands of the `pcp-protocol` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum PcpProtocolSubcommand {
	Run,
	#[clap(subcommand)]
	Client(PcpProtocolClientSubcommand),
}

/// Implement the `From` trait for `PcpProtocol` to convert it into a `PcpProtocolSubcommand`.
impl From<PcpProtocol> for PcpProtocolSubcommand {
	fn from(client: PcpProtocol) -> Self {
		client.command.unwrap_or(PcpProtocolSubcommand::Run)
	}
}

/// Implement the `PcpProtocol` CLI.
impl PcpProtocol {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpProtocolSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpProtocolSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpProtocolSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpProtocolSubcommand::Run => {
				println!("pcp-protocol is under development. Please check back later.");
			}
			PcpProtocolSubcommand::Client(client) => {
				client.execute().await?;
			}
		}
		Ok(())
	}
}
