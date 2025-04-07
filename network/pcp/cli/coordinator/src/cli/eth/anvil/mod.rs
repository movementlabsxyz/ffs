pub mod up;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Anvil {
	/// Brings-up a PCP network on Anvil.
	#[clap(subcommand)]
	Up(up::or_file::Up),
}

impl Anvil {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Anvil::Up(up) => up.execute().await?,
		}
		Ok(())
	}
}
