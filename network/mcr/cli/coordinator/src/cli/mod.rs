use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum McrNetworkCoordinator {}

impl McrNetworkCoordinator {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		println!("mcr-network-coordinator is under development. Please check back later.");

		Ok(())
	}
}
