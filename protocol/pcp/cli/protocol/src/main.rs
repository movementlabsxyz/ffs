use clap::*;
use dotenv::dotenv;
use pcp_protocol::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let pcp_protocol = cli::PcpProtocol::parse();
	pcp_protocol.execute().await?;
	Ok(())
}
