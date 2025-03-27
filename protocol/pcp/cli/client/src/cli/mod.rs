pub mod eth;
pub mod post_commitment;
pub mod deploy;
use clap::{Parser, Subcommand};

/// The `pcp-protocol-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpProtocolClient {
	#[clap(subcommand)]
	command: Option<PcpProtocolClientSubcommand>,
}

/// The subcommands of the `pcp-protocol-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum PcpProtocolClientSubcommand {
	Run,
	#[clap(subcommand)]
	Eth(eth::Eth),
	/// Post a commitment to an PCP implementation
	PostCommitment(post_commitment::PostCommitment),
	/// Deploy PCP contracts using deployer-core
	#[clap(subcommand)]
	Deploy(deploy::Deploy),
}

/// Implement the `From` trait for `PcpProtocolClient` to convert it into a `PcpProtocolClientSubcommand`.
impl From<PcpProtocolClient> for PcpProtocolClientSubcommand {
	fn from(client: PcpProtocolClient) -> Self {
		client.command.unwrap_or(PcpProtocolClientSubcommand::Run)
	}
}

/// Implement the `PcpProtocolClient` CLI.
impl PcpProtocolClient {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpProtocolClientSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpProtocolClientSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpProtocolClientSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpProtocolClientSubcommand::Run => {
				println!("pcp-protocol-client is under development. Please check back later.");
			}
			PcpProtocolClientSubcommand::Eth(eth) => eth.execute().await?,
			PcpProtocolClientSubcommand::PostCommitment(post_commitment) => {
				post_commitment.execute().await?
			}
			PcpProtocolClientSubcommand::Deploy(deploy) => deploy.execute().await?,
		}
		Ok(())
	}
}
