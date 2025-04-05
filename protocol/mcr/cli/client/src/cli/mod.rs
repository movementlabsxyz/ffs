pub mod eth;
use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

/// The `mcr-protocol-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrProtocolClient {
	#[clap(subcommand)]
	command: Option<McrProtocolClientSubcommand>,
}

/// The subcommands of the `mcr-protocol-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum McrProtocolClientSubcommand {
	/// ???
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Ethereum-specific commands of the protocol, such as staking and committing
	#[clap(subcommand)]
	Eth(eth::Eth),
}

/// Implement the `From` trait for `McrProtocolClient` to convert it into a `McrProtocolClientSubcommand`.
impl From<McrProtocolClient> for McrProtocolClientSubcommand {
	fn from(client: McrProtocolClient) -> Self {
		client
			.command
			.unwrap_or(McrProtocolClientSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `McrProtocolClient` CLI.
impl McrProtocolClient {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrProtocolClientSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrProtocolClientSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrProtocolClientSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrProtocolClientSubcommand::Markdown(markdown) => {
				markdown.execute::<McrProtocolClient>().await?;
			}
			McrProtocolClientSubcommand::Eth(eth) => eth.execute().await?,
		}
		Ok(())
	}
}
