pub mod anvil;
pub mod live;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Eth {
	/// ???
	#[clap(subcommand)]
	Anvil(anvil::Anvil),
	/// ???
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
