pub mod up;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Live {
	/// Brings-up an MCR network on a live Ethereum network.
	Up(up::Up),
}

impl Live {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Live::Up(up) => up.execute().await?,
		}
		Ok(())
	}
}
