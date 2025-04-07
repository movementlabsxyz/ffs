use clap::Parser;
use mcr_network::cli::McrNetworkSubcommand;
use mcr_protocol::cli::McrProtocolSubcommand;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Mcr {
	/// Subcommands for bringing-up an MCR network.
	#[clap(subcommand)]
	Network(McrNetworkSubcommand),
	/// Subcommands for the MCR protocol.
	#[clap(subcommand)]
	Protocol(McrProtocolSubcommand),
}

impl Mcr {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Mcr::Network(network) => {
				network.execute().await?;
			}
			Mcr::Protocol(protocol) => {
				protocol.execute().await?;
			}
		}

		Ok(())
	}
}
