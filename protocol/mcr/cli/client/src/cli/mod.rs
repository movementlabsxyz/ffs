pub mod eth;
pub mod post_commitment;
pub mod deploy;
pub mod check_commitment;
pub mod check_postconfirmation;
pub mod stake;
use clap::{Parser, Subcommand};
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_types::block_commitment::BlockCommitment;

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
	/// Post a commitment
	PostCommitment(post_commitment::PostCommitment),
	/// Check a commitment for a given height and attester
	CheckCommitment(check_commitment::CheckCommitment),
	/// Check postconfirmation for a height
	CheckPostconfirmation(check_postconfirmation::CheckPostconfirmation),
	/// Deploy MCR contracts using deployer-core
	#[clap(subcommand)]
	Deploy(deploy::Deploy),
	/// Stake MOVE tokens
	Stake(stake::Stake),
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
			McrProtocolClientSubcommand::CheckCommitment(check_commitment) => {
				check_commitment.execute().await?
			}
			McrProtocolClientSubcommand::CheckPostconfirmation(check_postconfirmation) => {
				check_postconfirmation.execute().await?
			}
			McrProtocolClientSubcommand::Deploy(deploy) => deploy.execute().await?,
			McrProtocolClientSubcommand::Stake(stake) => stake.execute().await?,
		}
		Ok(())
	}
}
