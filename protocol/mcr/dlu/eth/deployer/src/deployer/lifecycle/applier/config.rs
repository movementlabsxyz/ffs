use super::Arguments;
use super::{
	super::{artifacts, ForgeDeployer, Lifecycle},
	Applier, ScriptArgs,
};
use crate::contracts::ContractWorkspace;
use clap::Parser;
use jsonlvar::Jsonl;
use lifecycle::{ApplyOperations, LifecycleApplyFrontend, LifecycleError, LifecycleOperations};
use orfile::Orfile;
use secure_signer_loader::identifiers::SignerIdentifier;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Jsonl, Orfile)]
#[clap(help_expected = true)]
pub struct Config {
	/// The signer identifier.
	#[arg(long)]
	pub signer_identifier: SignerIdentifier,
	/// The fork url for deployment.
	#[arg(long)]
	pub fork_url: String,
	/// The deployer config.
	#[orfile(config)]
	#[clap(flatten)]
	pub script_args: ScriptArgs,
	/// The JSONL prefix to give to the output from the deployer.
	#[arg(long)]
	pub jsonl_prefix: Option<String>,
}

impl Config {
	/// Creates a new MCR client configuration.
	pub fn new(
		signer_identifier: SignerIdentifier,
		fork_url: String,
		script_args: ScriptArgs,
		jsonl_prefix: Option<String>,
	) -> Self {
		Config { signer_identifier, fork_url, script_args, jsonl_prefix }
	}

	/// Returns a new instance of [Config] which is designed to be filled in some fields.
	pub fn to_be_filled() -> Result<Self, anyhow::Error> {
		Ok(Self {
			signer_identifier: SignerIdentifier::from_str(
				"local::0000000000000000000000000000000000000000",
			)
			.map_err(|e| anyhow::anyhow!("failed to parse signer identifier: {}", e))?,
			fork_url: "http://localhost:8545".to_string(),
			script_args: ScriptArgs::to_be_filled(),
			jsonl_prefix: None,
		})
	}
	pub async fn build(&self) -> Result<Applier, anyhow::Error> {
		// get the raw private key
		let raw_key = self.signer_identifier.try_raw_private_key()?;
		let raw_private_key = format!("0x{}", hex::encode(raw_key));

		// form the forge deployer
		let forge_deployer = ForgeDeployer {
			workspace: ContractWorkspace::try_temp()?.into(),
			raw_private_key,
			fork_url: self.fork_url.clone(),
			jsonl_prefix: self.jsonl_prefix.clone(),
		};

		// form the lifecyle
		let lifecyle = Lifecycle { forge_deployer };

		// get the applier from the lifecyle
		let applier = lifecyle
			.applier_of(self.script_args.args.clone(), self.script_args.artifacts.clone())
			.await?;

		Ok(applier)
	}

	pub async fn apply(&self) -> Result<artifacts::output::Artifacts, anyhow::Error> {
		// build the applier
		let applier = self.build().await?;

		// run the applier
		let artifacts = applier.apply().await?;

		Ok(artifacts)
	}
}

impl LifecycleApplyFrontend for Config {
	type Lifecycle = Lifecycle;

	async fn to_applier_parts(
		&self,
	) -> Result<(Self::Lifecycle, Arguments, artifacts::input::Artifacts), LifecycleError> {
		// get the raw private key
		let raw_key = self
			.signer_identifier
			.try_raw_private_key()
			.map_err(|e| LifecycleError::Config(e.into()))?;
		let raw_private_key = format!("0x{}", hex::encode(raw_key));

		// form the forge deployer
		let forge_deployer = ForgeDeployer {
			workspace: ContractWorkspace::try_temp()
				.map_err(|e| LifecycleError::Config(e.into()))?
				.into(),
			raw_private_key,
			fork_url: self.fork_url.clone(),
			jsonl_prefix: self.jsonl_prefix.clone(),
		};

		// form the lifecyle
		let lifecyle = Lifecycle { forge_deployer };

		// get the args
		let args = self.script_args.args.clone();

		// get the artifacts
		let artifacts = self.script_args.artifacts.clone();

		Ok((lifecyle, args, artifacts))
	}
}

impl LifecycleApplyFrontend for or_file::Config {
	type Lifecycle = Lifecycle;

	async fn to_applier_parts(
		&self,
	) -> Result<(Self::Lifecycle, Arguments, artifacts::input::Artifacts), LifecycleError> {
		let config = self.clone().resolve().await.map_err(|e| LifecycleError::Config(e.into()))?;

		config.to_applier_parts().await
	}
}
