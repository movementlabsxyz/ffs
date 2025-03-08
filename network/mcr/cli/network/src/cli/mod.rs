use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum McrNetwork {}

impl McrNetwork {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		println!("mcr-network is under development. Please check back later.");

		Ok(())
	}
}
