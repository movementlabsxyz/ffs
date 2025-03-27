use kestrel::State;
pub use mcr_dlu_eth_deployer_core::dev::{artifacts::Artifacts, config::Config};
use network_anvil_component_core::{lifecycle::up::Up as AnvilUp, util::parser::AnvilData};
use secure_signer_loader::identifiers::{local::Local, SignerIdentifier};

/// Up struct for managing the MCR deployment process against Anvil.
pub struct Up {
	anvil_deploy: AnvilUp,
	config: Config,
	artifacts: State<Artifacts>,
}

impl Up {
	pub fn new(config: Config) -> Self {
		Up { anvil_deploy: AnvilUp::new(), config, artifacts: State::new() }
	}

	pub fn anvil_data(&self) -> &State<AnvilData> {
		&self.anvil_deploy.anvil_data()
	}

	pub fn artifacts(&self) -> &State<Artifacts> {
		&self.artifacts
	}

	pub async fn run(mut self) -> Result<(), anyhow::Error> {
		let anvil_data = self.anvil_deploy.anvil_data().clone();

		// Start the Anvil deployment process.
		let deployer_future = kestrel::task(async move { self.anvil_deploy.run().await });

		// Wait on the anvil data
		let anvil_data = anvil_data.read().wait_for().await;

		// Overwrite whatever was in self.config with Anvil data
		self.config.fork_url = "http://localhost:8545".to_string();

		// get the deployer signer from index 0 and strip the 0x
		let private_key_hex_bytes = anvil_data
			.private_keys
			.get(0)
			.cloned()
			.ok_or(anyhow::anyhow!("no signers found"))?;
		let private_key_hex_bytes = private_key_hex_bytes
			.strip_prefix("0x")
			.unwrap_or(&private_key_hex_bytes)
			.to_string();
		self.config.signer_identifier = SignerIdentifier::Local(Local { private_key_hex_bytes });

		self.config.deployer_config.contract_admin =
			anvil_data.signers.get(1).cloned().ok_or(anyhow::anyhow!("no signers found"))?;

		// build the deployer
		let deployer = self.config.build()?;
		let artifacts = deployer.deploy().await?;

		// for composability, set the artifacts in the state
		self.artifacts.write().set(artifacts).await;

		// Wait for the anvil deployer to finish
		deployer_future.await??;

		Ok(())
	}
}
