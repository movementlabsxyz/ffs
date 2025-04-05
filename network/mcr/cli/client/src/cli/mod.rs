use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

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
	/// ???
	#[clap(subcommand)]
	Markdown(Markdown),
}

/// Implement the `From` trait for `McrNetworkClient` to convert it into a `McrNetworkClientSubcommand`.
impl From<McrNetworkClient> for McrNetworkClientSubcommand {
	fn from(client: McrNetworkClient) -> Self {
		client
			.command
			.unwrap_or(McrNetworkClientSubcommand::Markdown(Markdown::default()))
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
			McrNetworkClientSubcommand::Markdown(markdown) => {
				markdown.execute::<McrNetworkClient>().await?;
			}
		}
		Ok(())
	}
}
