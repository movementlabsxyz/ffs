use clap::Parser;
use mcr_network::cli::McrNetworkSubcommand;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsDev {
	#[clap(subcommand)]
	Network(McrNetworkSubcommand),
}

impl FfsDev {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			FfsDev::Network(network) => {
				network.execute().await?;
			}
		}

		Ok(())
	}
}
