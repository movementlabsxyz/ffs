pub mod mcr;

use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsDev {
	#[clap(subcommand)]
	Mcr(mcr::Mcr),
}

impl FfsDev {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			FfsDev::Mcr(mcr) => {
				mcr.execute().await?;
			}
		}

		Ok(())
	}
}
