pub mod emln;
pub mod exln;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Eth {
	/// Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be externalized
	#[clap(subcommand)]
	Exln(exln::Exln),
	/// Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be internalized
	#[clap(subcommand)]
	Emln(emln::Emln),
}

impl Eth {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Self::Exln(exln) => exln.execute().await,
			Self::Emln(emln) => emln.execute().await,
		}
	}
}
