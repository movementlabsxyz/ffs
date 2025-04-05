use clap::Parser;
use pcp_network::cli::PcpNetworkSubcommand;
use pcp_protocol::cli::PcpProtocolSubcommand;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Pcp {
	/// Subcommands for the PCP network.
	#[clap(subcommand)]
	Network(PcpNetworkSubcommand),
	/// Subcommands for the PCP protocol.
	#[clap(subcommand)]
	Protocol(PcpProtocolSubcommand),
}

impl Pcp {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Pcp::Network(network) => {
				network.execute().await?;
			}
			Pcp::Protocol(protocol) => {
				protocol.execute().await?;
			}
		}

		Ok(())
	}
}
