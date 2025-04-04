use kestrel::State;
pub use mcr_protocol_deployer_eth_core::{applier::config::Config, artifacts::output::Artifacts};

/// Up struct for managing the MCR deployment process against Anvil.
pub struct Up {
	config: Config,
	artifacts: State<Artifacts>,
}

impl Up {
	pub fn new(config: Config) -> Self {
		Up { config, artifacts: State::new() }
	}

	pub fn artifacts(&self) -> &State<Artifacts> {
		&self.artifacts
	}

	pub async fn run(self) -> Result<(), anyhow::Error> {
		// apply the config to the live network
		let artifacts = self.config.apply().await?;

		// for composability, set the artifacts in the state
		self.artifacts.write().set(artifacts).await;

		// for now just wait forever
		// todo: in the future we will make some health checks on the contract
		std::future::pending::<()>().await;

		Ok(())
	}
}
