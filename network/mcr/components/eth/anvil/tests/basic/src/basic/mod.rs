pub mod client;
pub mod config;

use anyhow::Context;
use client::{Act, Client};
use config::Config;
use mcr_network_anvil_component_core::dev::lifecycle::up::Up;
use mcr_protocol_client_eth_core::config::Config as EthConfig;
use mcr_protocol_deployer_eth_core::artifacts::output::Artifacts;
use mcr_types::block_commitment::Commitment;
use network_anvil_component_core::util::parser::AnvilData;
use secure_signer::key::TryFromCanonicalString;
use secure_signer_loader::identifiers::SignerIdentifier;
use tokio::time::Duration;
pub struct Basic {
	up: Up,
}

// todo: perhaps move into Up API.
pub struct UpState {
	pub anvil_data: AnvilData,
	pub artifacts: Artifacts,
}

impl UpState {
	pub fn try_to_default_mcr_protocol_client_config(&self) -> Result<EthConfig, anyhow::Error> {
		// todo: this should be retrieved from the anvil data.
		let rpc_url = "http://localhost:8545".to_string();
		let ws_url = "ws://localhost:8545".to_string();
		let chain_id = self.anvil_data.chain_id;
		let block_lead_tolerance = 100;

		// get the signer identifier
		let signer_identifier_hex_key = self.anvil_data.private_keys[0].clone();
		let canonical_identifier_string = format!(
			"local::{}",
			signer_identifier_hex_key
				.strip_prefix("0x")
				.context("invalid signer identifier")?
		);
		let signer_identifier =
			SignerIdentifier::try_from_canonical_string(&canonical_identifier_string)
				.map_err(|_| anyhow::anyhow!("invalid signer identifier"))?;

		Ok(EthConfig {
			mcr_contract_address: self.artifacts.mcr_proxy.clone(),
			rpc_url: rpc_url.clone(),
			ws_url: ws_url.clone(),
			chain_id: chain_id,
			signer_identifier: signer_identifier,
			run_commitment_admin_mode: false,
			gas_limit: 323924465909782,
			transaction_send_retries: 3,
			mcr_address: self.artifacts.mcr_proxy.clone(),
			block_lead_tolerance: block_lead_tolerance,
			move_token_address: self.artifacts.token_proxy.clone(),
			staking_address: self.artifacts.staking_proxy.clone(),
		})
	}

	pub async fn try_build_default_mcr_protocol_client(&self) -> Result<Client, anyhow::Error> {
		let mcr_protocol_client_config = self.try_to_default_mcr_protocol_client_config()?;
		let mcr_protocol_client = mcr_protocol_client_config.build().await?;
		Ok(Client::new(mcr_protocol_client))
	}
}

impl Basic {
	pub fn new(config: Config) -> Self {
		Basic { up: Up::new(config.up) }
	}

	pub async fn run(self) -> Result<(), anyhow::Error> {
		// clone the anvil data and artifacts from up
		let anvil_data = self.up.anvil_data().clone();
		let artifacts = self.up.artifacts().clone();

		let up_task = kestrel::task(async move { self.up.run().await });

		// wait for the anvil data and artifacts for up to 30 seconds
		println!("waiting for anvil data");
		let anvil_data = anvil_data.read().wait_for(Duration::from_secs(30)).await?;
		println!("waiting for artifacts");
		let artifacts = artifacts.read().wait_for(Duration::from_secs(30)).await?;
		println!("up state");
		let up_state = UpState { anvil_data, artifacts };

		// get the mcr protocol client
		let mcr_protocol_client = up_state.try_build_default_mcr_protocol_client().await?;

		// act with the client
		mcr_protocol_client.act(Act::PostCommitment(Commitment::default())).await?;

		// end the up task
		kestrel::end!(up_task)?;

		Ok(())
	}
}
