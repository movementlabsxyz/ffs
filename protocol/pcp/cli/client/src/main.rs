use clap::*;
use dotenv::dotenv;
use pcp_protocol_client::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let pcp_protocol_client = cli::PcpProtocolClient::parse();
	pcp_protocol_client.execute().await?;
	Ok(())
}
