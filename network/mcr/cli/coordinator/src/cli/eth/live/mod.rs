pub mod up;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Live {
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
