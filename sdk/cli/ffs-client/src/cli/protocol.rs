use clap::Subcommand;
use mcr_protocol_client::cli::McrProtocolClientSubcommand;
use pcp_protocol_client::cli::PcpProtocolClientSubcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Protocol {
	/// MCR protocol commands
	#[clap(subcommand)]
	Mcr(McrProtocolClientSubcommand),
	/// PCP protocol commands
	#[clap(subcommand)]
	Pcp(PcpProtocolClientSubcommand),
}

impl Protocol {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Protocol::Mcr(client) => {
				client.execute().await?;
			}
			Protocol::Pcp(client) => {
				client.execute().await?;
			}
		}

		Ok(())
	}
}
