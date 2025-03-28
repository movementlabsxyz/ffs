use clap::*;
use dotenv::dotenv;
use pcp_network_client::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let pcp_network_client = cli::PcpNetworkClient::parse();
	pcp_network_client.execute().await?;
	Ok(())
}
