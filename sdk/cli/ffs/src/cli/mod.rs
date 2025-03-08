use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Ffs {}

impl Ffs {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		println!("ffs is under development. Please check back later.");

		Ok(())
	}
}
