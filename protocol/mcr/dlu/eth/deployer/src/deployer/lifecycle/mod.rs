pub mod applier;
pub mod artifacts;
pub mod destroyer;

use crate::contracts::ContractWorkspace;
use lifecycle::{LifecycleError, LifecycleFrontend, LifecycleOperations};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ForgeDeployer {
	/// The contract workspace in which the deployment command will run.
	pub workspace: Arc<ContractWorkspace>,
	/// The raw private key used for deployment.
	pub raw_private_key: String,
	/// The fork url used for deployment.
	pub fork_url: String,
	/// The jsonl prefix to give to the output from the deployer.
	pub jsonl_prefix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Lifecycle {
	pub forge_deployer: ForgeDeployer,
}

impl LifecycleOperations for Lifecycle {
	type Applier = applier::Applier;
	type Destroyer = destroyer::Destroyer;

	async fn applier_of(
		&self,
		args: applier::Arguments,
		artifacts: artifacts::input::Artifacts,
	) -> Result<Self::Applier, LifecycleError> {
		Ok(applier::Applier {
			script_args: applier::ScriptArgs { args, artifacts },
			forge_deployer: self.forge_deployer.clone(),
		})
	}

	async fn destroyer_of(
		&self,
		args: destroyer::Arguments,
		artifacts: artifacts::output::Artifacts,
	) -> Result<Self::Destroyer, LifecycleError> {
		Ok(destroyer::Destroyer { args, artifacts, forge_deployer: self.forge_deployer.clone() })
	}
}

impl LifecycleFrontend for Lifecycle {
	type ApplyFrontend = applier::or_file::Config;
	type DestroyFrontend = destroyer::Config;
}
