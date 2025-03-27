use clap::Parser;
// use pcp_network::cli::PcpNetworkSubcommand;
use pcp_protocol::cli::PcpProtocolSubcommand;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Pcp {
	// #[clap(subcommand)]
	// Network(PcpNetworkSubcommand),
	#[clap(subcommand)]
	Protocol(PcpProtocolSubcommand),
}

impl Pcp {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			// Pcp::Network(network) => {
			// 	network.execute().await?;
			// }
			Pcp::Protocol(protocol) => {
				protocol.execute().await?;
			}
		}

		Ok(())
	}
}
