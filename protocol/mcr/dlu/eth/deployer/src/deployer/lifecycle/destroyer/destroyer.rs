use super::{
	super::{artifacts, ForgeDeployer},
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
	type InputArtifacts = artifacts::input::Artifacts;
	type OutputArtifacts = artifacts::output::Artifacts;

	async fn destroy(&self) -> Result<Self::InputArtifacts, LifecycleError> {
		todo!()
	}
}
