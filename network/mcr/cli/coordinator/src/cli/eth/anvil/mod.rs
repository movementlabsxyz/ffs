pub mod up;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Anvil {
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
