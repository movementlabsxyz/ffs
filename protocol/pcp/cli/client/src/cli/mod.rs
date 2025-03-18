pub mod eth;
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
	/// Post a commitment to a PCP implementation
	PostCommitment(PostCommitmentArgs),
}

#[derive(clap::Args)]
pub struct PostCommitmentArgs {
	/// Hex-encoded commitment
	#[clap(long, conflicts_with = "preimage_string", required_unless_present = "preimage_string")]
	commitment_hex: Option<String>,

	/// String to be hashed into a commitment
	#[clap(long, conflicts_with = "commitment_hex", required_unless_present = "commitment_hex")]
	preimage_string: Option<String>,
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
			PcpProtocolClientSubcommand::PostCommitment(args) => {
				// For now, just print what we would do
				if let Some(hex) = &args.commitment_hex {
					println!("Would post commitment to PCP from hex: {}", hex);
				} else if let Some(preimage) = &args.preimage_string {
					println!("Would hash and post commitment to PCP from preimage: {}", preimage);
				}
			}
		}
		Ok(())
	}
}
