use clap::*;
use dotenv::dotenv;
use mcr_network::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mcr_network = cli::McrNetwork::parse();
	mcr_network.execute().await?;
	Ok(())
}
