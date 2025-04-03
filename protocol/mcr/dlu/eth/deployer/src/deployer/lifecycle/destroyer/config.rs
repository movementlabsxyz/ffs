use super::super::{artifacts, Lifecycle};
use super::Arguments;
use crate::contracts::ContractWorkspace;
use crate::lifecycle::ForgeDeployer;
use clap::Parser;
use lifecycle::{LifecycleDestroyFrontend, LifecycleError};
use secure_signer::key::TryFromCanonicalString;
use secure_signer_loader::identifiers::SignerIdentifier;

#[derive(Debug, Clone, Parser)]
#[clap(help_expected = true)]
pub struct Config {
	/// The signer identifier.
	#[arg(value_parser = SignerIdentifier::try_from_canonical_string)]
	#[arg(long)]
	pub signer_identifier: SignerIdentifier,
	/// The fork url for deployment.
	#[arg(long)]
	pub fork_url: String,
	/// The artifacts to destroy.
	#[clap(flatten)]
	pub script_args: artifacts::output::Artifacts,
	/// The JSONL prefix to give to the output from the deployer.
	#[arg(long)]
	pub jsonl_prefix: Option<String>,
}

impl LifecycleDestroyFrontend for Config {
	type Lifecycle = Lifecycle;

	async fn to_destroyer_parts(
		&self,
	) -> Result<(Self::Lifecycle, Arguments, artifacts::output::Artifacts), LifecycleError> {
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
		let args = Arguments::new();

		// get the artifacts
		let artifacts = self.script_args.clone();

		Ok((lifecyle, args, artifacts))
	}
}
