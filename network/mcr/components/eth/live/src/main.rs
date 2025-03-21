use mcr_network_eth_live_component_core::dev::lifecycle::up::Up;
use mcr_protocol_deployer_eth_core::dev::config::Config;
use secure_signer_loader::identifiers::{local::Local, SignerIdentifier};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
	let mcr_config = Config {
		fork_url: "http://localhost:8545".to_string(),
		signer_identifier: SignerIdentifier::Local(Local {
			private_key_hex_bytes: "notreal".to_string(), // Replace with actual private key
		}), // Replace with actual identifier
		contract_admin: "0x...".to_string(), // Replace with actual admin address
		jsonl_prefix: None,
	};
	let deploy = Up::new(mcr_config);
	deploy.run().await?;
	Ok(())
}
