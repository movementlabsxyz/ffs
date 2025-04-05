use clap::Subcommand;
use mcr_protocol_client::cli::McrProtocolClientSubcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Protocol {
	/// MCR protocol commands
	#[clap(subcommand)]
	Mcr(McrProtocolClientSubcommand),
}

impl Protocol {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Protocol::Mcr(client) => {
				client.execute().await?;
			}
		}

		Ok(())
	}
}
