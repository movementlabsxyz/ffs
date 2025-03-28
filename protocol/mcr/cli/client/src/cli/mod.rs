pub mod eth;
pub mod post_commitment;
pub mod deploy;
use clap::{Parser, Subcommand};

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
	Run,
	/// ??? 
	#[clap(subcommand)]
	Eth(eth::Eth),
	/// Post a commitment to an MCR implementation
	PostCommitment(post_commitment::PostCommitment),
	/// Deploy MCR contracts using deployer-core
	#[clap(subcommand)]
	Deploy(deploy::Deploy),
}

/// Implement the `From` trait for `McrProtocolClient` to convert it into a `McrProtocolClientSubcommand`.
impl From<McrProtocolClient> for McrProtocolClientSubcommand {
	fn from(client: McrProtocolClient) -> Self {
		client.command.unwrap_or(McrProtocolClientSubcommand::Run)
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
			McrProtocolClientSubcommand::Run => {
				println!("mcr-protocol-client is under development. Please check back later.");
			}
			McrProtocolClientSubcommand::Eth(eth) => eth.execute().await?,
			McrProtocolClientSubcommand::PostCommitment(post_commitment) => {
				post_commitment.execute().await?
			}
			McrProtocolClientSubcommand::Deploy(deploy) => deploy.execute().await?,
		}
		Ok(())
	}
}
