pub mod anvil;
pub mod live;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Eth {
	/// Anvil-specific commands of the network coordinator, i.e., for bringing-up a PCP network on Anvil.
	#[clap(subcommand)]
	Anvil(anvil::Anvil),
	/// Live-Ethereum-based commands of the network coordinator, i.e., for bringing-up a PCP network on a live Ethereum network.
	#[clap(subcommand)]
	Live(live::Live),
}

impl Eth {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Eth::Anvil(anvil) => anvil.execute().await?,
			Eth::Live(live) => live.execute().await?,
		}
		Ok(())
	}
}
