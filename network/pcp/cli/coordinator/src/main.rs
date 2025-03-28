use clap::*;
use dotenv::dotenv;
use pcp_network_coordinator::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let pcp_network_coordinator = cli::PcpNetworkCoordinator::parse();
	pcp_network_coordinator.execute().await?;
	Ok(())
}
