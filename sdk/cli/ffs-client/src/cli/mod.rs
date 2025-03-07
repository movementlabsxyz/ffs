use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsClient {}

impl FfsClient {
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		println!("ffs-client is under development. Please check back later.");

		Ok(())
	}
}
