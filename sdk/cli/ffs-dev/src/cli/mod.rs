pub mod mcr;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsDev {
	#[clap(subcommand)]
	Markdown(Markdown),
	#[clap(subcommand)]
	Mcr(mcr::Mcr),
}

impl FfsDev {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			FfsDev::Markdown(markdown) => {
				markdown.execute::<Self>().await?;
			}
			FfsDev::Mcr(mcr) => {
				mcr.execute().await?;
			}
		}

		Ok(())
	}
}
