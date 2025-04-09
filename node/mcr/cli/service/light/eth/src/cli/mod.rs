pub mod emln;
pub mod exln;

use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

/// The `mcr-light-node-eth` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct McrLightNodeEth {
	#[clap(subcommand)]
	command: Option<McrLightNodeEthSubcommand>,
}

/// The subcommands of the `mcr-light-node-eth` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum McrLightNodeEthSubcommand {
	/// Generate markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Runs the embedded Ethereum MCR light node.
	#[clap(subcommand)]
	Emln(emln::or_file::Emln),
	/// Runs the externalized Ethereum MCR light node.
	#[clap(subcommand)]
	Exln(exln::or_file::Exln),
}

/// Implement the `From` trait for `McrLightNodeEth` to convert it into a `McrLightNodeEthSubcommand`.
impl From<McrLightNodeEth> for McrLightNodeEthSubcommand {
	fn from(coordinator: McrLightNodeEth) -> Self {
		coordinator
			.command
			.unwrap_or(McrLightNodeEthSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `McrLightNodeEth` CLI.
impl McrLightNodeEth {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: McrLightNodeEthSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `McrLightNodeEthSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl McrLightNodeEthSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			McrLightNodeEthSubcommand::Markdown(markdown) => {
				markdown.execute::<McrLightNodeEth>().await?;
			}
			McrLightNodeEthSubcommand::Emln(emln) => emln.execute().await?,
			McrLightNodeEthSubcommand::Exln(exln) => exln.execute().await?,
		}
		Ok(())
	}
}
