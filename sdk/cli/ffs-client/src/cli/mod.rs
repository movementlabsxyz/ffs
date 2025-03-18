use clap::Parser;
pub mod protocol;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsClient {
	#[clap(subcommand)]
	Protocol(protocol::Protocol),
}

impl FfsClient {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			FfsClient::Protocol(protocol) => {
				protocol.execute().await?;
			}
		}

		Ok(())
	}
}
