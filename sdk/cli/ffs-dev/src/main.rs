use clap::*;
use dotenv::dotenv;
use ffs_dev::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let ffs_dev = cli::FfsDev::parse();
	ffs_dev.execute().await?;
	Ok(())
}
