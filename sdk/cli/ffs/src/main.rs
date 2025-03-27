use clap::*;
use dotenv::dotenv;
use ffs::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let _ffs = cli::Ffs::parse();
	// _ffs.execute().await?;
	Ok(())
}
