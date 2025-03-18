pub mod eth;
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
pub enum McrProtocolClientSubcommand {
	Run,
	#[clap(subcommand)]
	Eth(eth::Eth),
	/// Post a commitment to an MCR implementation
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
			McrProtocolClientSubcommand::PostCommitment(args) => {
				// For now, just print what we would do
				if let Some(hex) = &args.commitment_hex {
					println!("Would post commitment from hex: {}", hex);
				} else if let Some(preimage) = &args.preimage_string {
					println!("Would hash and post commitment from preimage: {}", preimage);
				}
			}
		}
		Ok(())
	}
}
