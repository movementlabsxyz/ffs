use clap::Parser;
use mcr_network::cli::McrNetworkSubcommand;
use mcr_protocol::cli::McrProtocolSubcommand;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Mcr {
	/// ???
	#[clap(subcommand)]
	Network(McrNetworkSubcommand),
	/// The subcommands of the `mcr-protocol` CLI 2.
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
