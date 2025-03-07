use clap::*;
use dotenv::dotenv;
use ffs::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let ffs = cli::Ffs::parse();
	ffs.run().await?;
	Ok(())
}
