use clap::*;
use dotenv::dotenv;
use mcr_protocol::cli;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// Run the CLI.
	let mcr_protocol = cli::McrProtocol::parse();
	mcr_protocol.execute().await?;
	Ok(())
}
