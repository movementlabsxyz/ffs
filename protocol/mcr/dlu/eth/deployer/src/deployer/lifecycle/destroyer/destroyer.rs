use super::{
	super::{applier::Applier, artifacts, ForgeDeployer},
	Arguments,
};
use lifecycle::{DestroyOperations, LifecycleError};

#[derive(Debug, Clone)]
pub struct Destroyer {
	pub args: Arguments,
	pub artifacts: artifacts::output::Artifacts,
	pub forge_deployer: ForgeDeployer,
}

impl DestroyOperations for Destroyer {
	type DestroyArguments = Arguments;
	type Applier = Applier;

	async fn destroy(&self) -> Result<artifacts::output::Artifacts, LifecycleError> {
		todo!()
	}
}
