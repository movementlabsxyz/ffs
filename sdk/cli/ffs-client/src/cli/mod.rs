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
		println!("ffs-client is under development. Please check back later.");

		Ok(())
	}
}
