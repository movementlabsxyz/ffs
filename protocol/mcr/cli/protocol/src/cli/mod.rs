use clap::{Parser, Subcommand};
use mcr_protocol_client::cli::McrProtocolClientSubcommand;

/// The `mcr-protocol` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrProtocol {
	#[clap(subcommand)]
	command: Option<McrProtocolSubcommand>,
}

/// The subcommands of the `mcr-protocol` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrProtocolSubcommand {
	Run,
	#[clap(subcommand)]
	Client(McrProtocolClientSubcommand),
}

/// Implement the `From` trait for `McrProtocol` to convert it into a `McrProtocolSubcommand`.
impl From<McrProtocol> for McrProtocolSubcommand {
	fn from(client: McrProtocol) -> Self {
		client.command.unwrap_or(McrProtocolSubcommand::Run)
	}
}

/// Implement the `McrProtocol` CLI.
impl McrProtocol {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrProtocolSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrProtocolSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrProtocolSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrProtocolSubcommand::Run => {
				println!("mcr-protocol is under development. Please check back later.");
			}
			McrProtocolSubcommand::Client(client) => {
				client.execute().await?;
			}
		}
		Ok(())
	}
}
