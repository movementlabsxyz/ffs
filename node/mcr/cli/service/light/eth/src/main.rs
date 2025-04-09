use clap::*;
use dotenv::dotenv;
use mcr_light_node_eth::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mcr_light_node_eth = cli::McrLightNodeEth::parse();
	mcr_light_node_eth.execute().await?;
	Ok(())
}
