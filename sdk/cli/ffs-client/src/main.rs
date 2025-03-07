use clap::*;
use dotenv::dotenv;
use ffs_client::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let ffs_client = cli::FfsClient::parse();
	ffs_client.run().await?;
	Ok(())
}
