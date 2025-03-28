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

	#[error("method is unsupported: {0}")]
	Unsupported(#[source] Box<dyn std::error::Error + Send + Sync>),
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
	type Applier: ApplyOperations;
	type DestroyArguments;

	fn destroy(
		&self,
	) -> impl Future<Output = Result<<Self::Applier as ApplyOperations>::OutputArtifacts, LifecycleError>>;
}

/// A lifecycle must be able to create an applier and a destroyer from itself and its arguments and input artifacts
pub trait LifecycleOperations: Sized {
	type Applier: ApplyOperations;
	type Destroyer: DestroyOperations<Applier = Self::Applier>;

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
		artifacts: <<Self as LifecycleOperations>::Applier as ApplyOperations>::OutputArtifacts,
	) -> impl Future<Output = Result<Self::Destroyer, LifecycleError>>;
}

/// A frontend must be able to convert itself into a tuple of the lifecycle, the arguments, and the input artifacts
pub trait LifecycleApplyFrontend: Sized {
	type Lifecycle: LifecycleOperations;

	/// Convert the frontend into a tuple of the lifecycle, the arguments, and the input artifacts
	fn to_applier_parts(&self) -> impl Future<Output = Result<(Self::Lifecycle, <<Self::Lifecycle as LifecycleOperations>::Applier as ApplyOperations>::ApplyArguments, <<Self::Lifecycle as LifecycleOperations>::Applier as ApplyOperations>::InputArtifacts), LifecycleError>>;
}

pub trait LifecycleDestroyFrontend: Sized {
	type Lifecycle: LifecycleOperations;

	/// Convert the frontend into a tuple of the lifecycle, the arguments, and the output artifacts
	fn to_destroyer_parts(&self) -> impl Future<Output = Result<(Self::Lifecycle, <<Self::Lifecycle as LifecycleOperations>::Destroyer as DestroyOperations>::DestroyArguments, <<Self::Lifecycle as LifecycleOperations>::Applier as ApplyOperations>::OutputArtifacts), LifecycleError>>;
}

/// A frontend must be able to apply and destroy itself
pub trait LifecycleFrontend: LifecycleOperations {
	type ApplyFrontend: LifecycleApplyFrontend<Lifecycle = Self>;
	type DestroyFrontend: LifecycleDestroyFrontend<Lifecycle = Self>;

	/// Applies the frontend via the lifecycle
	fn apply(
		frontend: &Self::ApplyFrontend,
	) -> impl Future<
		Output = Result<
			<<Self as LifecycleOperations>::Applier as ApplyOperations>::OutputArtifacts,
			LifecycleError,
		>,
	> {
		async move {
			let (lifecycle, args, artifacts) = frontend.to_applier_parts().await?;
			let applier = lifecycle.applier_of(args, artifacts).await?;
			let artifacts = applier.apply().await?;
			Ok(artifacts)
		}
	}

	/// Destroys the frontend via the lifecycle
	fn destroy(
		frontend: &Self::DestroyFrontend,
	) -> impl Future<
		Output = Result<
			<<Self as LifecycleOperations>::Applier as ApplyOperations>::OutputArtifacts,
			LifecycleError,
		>,
	> {
		async move {
			let (lifecycle, args, artifacts) = frontend.to_destroyer_parts().await?;
			let destroyer = lifecycle.destroyer_of(args, artifacts).await?;
			let artifacts = destroyer.destroy().await?;
			Ok(artifacts)
		}
	}
}
