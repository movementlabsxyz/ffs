use std::future::Future;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
	#[error("failed to bring service up: {0}")]
	Up(#[source] Box<dyn std::error::Error + Send + Sync>),

	#[error("failed to bring service down: {0}")]
	Down(#[source] Box<dyn std::error::Error + Send + Sync>),

	#[error("invalid service configuration: {0}")]
	Config(#[source] Box<dyn std::error::Error + Send + Sync>),

	#[error("unsupported service operation: {0}")]
	Unsupported(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A service must be able to be created from a set of arguments and input artifacts
/// and must be able to bring itself up and return the output artifacts
pub trait UpOperations: Sized {
	type InputArtifacts: TryFrom<Self::OutputArtifacts>;
	type OutputArtifacts;
	type UpArguments;

	fn up(&self) -> impl Future<Output = Result<Self::OutputArtifacts, ServiceError>>;
}

/// A service must be able to be created from a set of arguments and output artifacts
/// and must be able to bring itself down and return the input artifacts
pub trait DownOperations: Sized {
	type Upper: UpOperations;
	type DownArguments;

	fn down(
		&self,
	) -> impl Future<Output = Result<<Self::Upper as UpOperations>::OutputArtifacts, ServiceError>>;
}

/// A service must be able to create an upper and a downer from itself and its arguments and input artifacts
pub trait ServiceOperations: Sized {
	type Upper: UpOperations;
	type Downer: DownOperations<Upper = Self::Upper>;

	/// Creates an upper for the service
	fn upper_of(
		&self,
		args: <<Self as ServiceOperations>::Upper as UpOperations>::UpArguments,
		artifacts: <<Self as ServiceOperations>::Upper as UpOperations>::InputArtifacts,
	) -> impl Future<Output = Result<Self::Upper, ServiceError>>;

	/// Creates a downer for the service
	fn downer_of(
		&self,
		args: <<Self as ServiceOperations>::Downer as DownOperations>::DownArguments,
		artifacts: <<Self as ServiceOperations>::Upper as UpOperations>::OutputArtifacts,
	) -> impl Future<Output = Result<Self::Downer, ServiceError>>;
}

/// A frontend must be able to convert itself into a tuple of the service, the arguments, and the input artifacts
pub trait ServiceUpFrontend: Sized {
	type Service: ServiceOperations;

	/// Convert the frontend into a tuple of the service, the arguments, and the input artifacts
	fn to_upper_parts(
		&self,
	) -> impl Future<
		Output = Result<
			(
				Self::Service,
				<<Self::Service as ServiceOperations>::Upper as UpOperations>::UpArguments,
				<<Self::Service as ServiceOperations>::Upper as UpOperations>::InputArtifacts,
			),
			ServiceError,
		>,
	>;
}

pub trait ServiceDownFrontend: Sized {
	type Service: ServiceOperations;

	/// Convert the frontend into a tuple of the service, the arguments, and the output artifacts
	fn to_downer_parts(
		&self,
	) -> impl Future<
		Output = Result<
			(
				Self::Service,
				<<Self::Service as ServiceOperations>::Downer as DownOperations>::DownArguments,
				<<Self::Service as ServiceOperations>::Upper as UpOperations>::OutputArtifacts,
			),
			ServiceError,
		>,
	>;
}

/// A frontend must be able to bring itself up and down
pub trait ServiceFrontend: ServiceOperations {
	type UpFrontend: ServiceUpFrontend<Service = Self>;
	type DownFrontend: ServiceDownFrontend<Service = Self>;

	/// Brings the frontend up via the service
	fn up(
		frontend: Self::UpFrontend,
	) -> impl Future<
		Output = Result<
			<<Self as ServiceOperations>::Upper as UpOperations>::OutputArtifacts,
			ServiceError,
		>,
	> {
		async move {
			let (service, args, artifacts) = frontend.to_upper_parts().await?;
			let upper = service.upper_of(args, artifacts).await?;
			let artifacts = upper.up().await?;
			Ok(artifacts)
		}
	}

	/// Brings the frontend down via the service
	fn down(
		frontend: Self::DownFrontend,
	) -> impl Future<
		Output = Result<
			<<Self as ServiceOperations>::Upper as UpOperations>::OutputArtifacts,
			ServiceError,
		>,
	> {
		async move {
			let (service, args, artifacts) = frontend.to_downer_parts().await?;
			let downer = service.downer_of(args, artifacts).await?;
			let artifacts = downer.down().await?;
			Ok(artifacts)
		}
	}
}
