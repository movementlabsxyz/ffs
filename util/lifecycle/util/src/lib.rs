use std::future::Future;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LifecycleError {
	#[error("failed to apply lifecycle: {0}")]
	Apply(#[source] Box<dyn std::error::Error + Send + Sync>),

	#[error("failed to destroy lifecycle: {0}")]
	Destroy(#[source] Box<dyn std::error::Error + Send + Sync>),

	#[error("invalid lifecycle configuration: {0}")]
	Config(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// An applier must be able to be created from a set of arguments and input artifacts
/// and must be able to apply itself to the input artifacts and return the output artifacts
pub trait ApplyOperations: Sized {
	type InputArtifacts: TryFrom<Self::OutputArtifacts>;
	type OutputArtifacts;
	type ApplyArguments;

	fn apply(&self) -> impl Future<Output = Result<Self::OutputArtifacts, LifecycleError>>;
}

/// A destroyer must be able to be created from a set of arguments and output artifacts
/// and must be able to destroy itself and return the input artifacts
pub trait DestroyOperations: Sized {
	type InputArtifacts;
	type OutputArtifacts;
	type DestroyArguments;

	fn destroy(&self) -> impl Future<Output = Result<Self::InputArtifacts, LifecycleError>>;
}

pub trait LifecycleOperations {
	type Applier: ApplyOperations;
	type Destroyer: DestroyOperations;

	/// Creates an applier for the lifecycle
	fn applier_of(
		&self,
		args: <<Self as LifecycleOperations>::Applier as ApplyOperations>::ApplyArguments,
		artifacts: <<Self as LifecycleOperations>::Applier as ApplyOperations>::InputArtifacts,
	) -> impl Future<Output = Result<Self::Applier, LifecycleError>>;

	/// Creates a destroyer for the lifecycle
	fn destroyer_of(
		&self,
		args: <<Self as LifecycleOperations>::Destroyer as DestroyOperations>::DestroyArguments,
		artifacts: <<Self as LifecycleOperations>::Destroyer as DestroyOperations>::OutputArtifacts,
	) -> impl Future<Output = Result<Self::Destroyer, LifecycleError>>;
}
