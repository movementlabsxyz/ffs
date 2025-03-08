use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum McrNetworkClient {}

impl McrNetworkClient {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		println!("mcr-network-client is under development. Please check back later.");

		Ok(())
	}
}
