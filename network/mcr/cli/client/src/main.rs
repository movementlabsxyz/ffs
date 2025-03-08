use clap::*;
use dotenv::dotenv;
use mcr_network_client::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mcr_network_client = cli::McrNetworkClient::parse();
	mcr_network_client.execute().await?;
	Ok(())
}
