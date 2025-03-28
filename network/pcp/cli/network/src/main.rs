use clap::*;
use dotenv::dotenv;
use pcp_network::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let pcp_network = cli::PcpNetwork::parse();
	pcp_network.execute().await?;
	Ok(())
}
