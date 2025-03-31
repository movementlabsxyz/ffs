pub mod mcr;
pub mod pcp;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum FfsDev {
	/// Generate CLI documentation
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Manage MCR
	#[clap(subcommand)]
	Mcr(mcr::Mcr),
	/// Manage PCP
	#[clap(subcommand)]
	Pcp(pcp::Pcp),
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
			FfsDev::Pcp(pcp) => {
				pcp.execute().await?;
			}
		}

		Ok(())
	}
}
