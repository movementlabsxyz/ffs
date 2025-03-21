use kestrel::State;
use mcr_protocol_deployer_eth_core::dev::config::Config;
use network_anvil_component_core::{
	lifecycle::deploy::Deploy as AnvilDeploy, util::parser::AnvilData,
};
use secure_signer_loader::identifiers::{local::Local, SignerIdentifier};

/// Deploy struct for managing the MCR deployment process against Anvil.
pub struct Deploy {
	anvil_deploy: AnvilDeploy,
	config: Config,
}

impl Deploy {
	pub fn new(config: Config) -> Self {
		Deploy { anvil_deploy: AnvilDeploy::new(), config }
	}

	pub fn anvil_data(&self) -> &State<AnvilData> {
		&self.anvil_deploy.anvil_data()
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

		self.config.contract_admin =
			anvil_data.signers.get(1).cloned().ok_or(anyhow::anyhow!("no signers found"))?;

		// build the deployer
		let deployer = self.config.build()?;
		deployer.deploy().await?;

		// Wait for the anvil deployer to finish
		deployer_future.await??;

		Ok(())
	}
}
