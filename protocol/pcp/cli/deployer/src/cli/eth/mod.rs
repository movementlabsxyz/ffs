pub mod deploy;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Eth {
	Deploy(deploy::Deploy),
}

impl Eth {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Eth::Deploy(cmd) => cmd.execute().await?,
		}
		Ok(())
	}
}
