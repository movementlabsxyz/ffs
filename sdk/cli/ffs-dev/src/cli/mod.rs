use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum FfsDev {}

impl FfsDev {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		println!("ffs-dev is under development. Please check back later.");

		Ok(())
	}
}
