use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
use mcr_protocol_client::cli::McrProtocolClientSubcommand;
use mcr_protocol_deployer::cli::McrProtocolDeployerSubcommand;
/// The `mcr-protocol` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrProtocol {
	#[clap(subcommand)]
	command: Option<McrProtocolSubcommand>,
}

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrProtocolSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// The client-specific commands of the MCR protocol.
	#[clap(subcommand)]
	Client(McrProtocolClientSubcommand),
	/// The deployer-specific commands of the MCR protocol.
	#[clap(subcommand)]
	Deployer(McrProtocolDeployerSubcommand),
}

/// Implement the `From` trait for `McrProtocol` to convert it into a `McrProtocolSubcommand`.
impl From<McrProtocol> for McrProtocolSubcommand {
	fn from(client: McrProtocol) -> Self {
		client.command.unwrap_or(McrProtocolSubcommand::Client(
			McrProtocolClientSubcommand::Markdown(Markdown::default()),
		))
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
			McrProtocolSubcommand::Markdown(markdown) => {
				markdown.execute::<McrProtocol>().await?;
			}
			McrProtocolSubcommand::Client(client) => {
				client.execute().await?;
			}
			McrProtocolSubcommand::Deployer(deployer) => {
				deployer.execute().await?;
			}
		}
		Ok(())
	}
}
