use crate::contracts::ContractWorkspace;
use crate::dev::deployer::{DeployConfig, Deployer};
use clap::Parser;
use jsonlvar::Jsonl;
use secure_signer::key::TryFromCanonicalString;
use secure_signer_loader::identifiers::SignerIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Jsonl)]
#[clap(help_expected = true)]
pub struct Config {
	/// The signer identifier.
	#[arg(value_parser = SignerIdentifier::try_from_canonical_string)]
	#[arg(long)]
	pub signer_identifier: SignerIdentifier,
	/// The fork url for deployment.
	#[arg(long)]
	pub fork_url: String,
	/// The deployer config.
	#[clap(flatten)]
	pub deployer_config: DeployConfig,
	/// The JSONL prefix to give to the output from the deployer.
	#[arg(long)]
	pub jsonl_prefix: Option<String>,
}

impl Config {
	/// Creates a new PCP client configuration.
	pub fn new(
		signer_identifier: SignerIdentifier,
		fork_url: String,
		deployer_config: DeployConfig,
		jsonl_prefix: Option<String>,
	) -> Self {
		Config { signer_identifier, fork_url, deployer_config, jsonl_prefix }
	}

	pub fn build(&self) -> Result<Deployer, anyhow::Error> {
		let raw_key = self.signer_identifier.try_raw_private_key()?;
		let raw_private_key = format!("0x{}", hex::encode(raw_key));

		Ok(Deployer {
			workspace: ContractWorkspace::try_temp_tip()?,
			raw_private_key,
			fork_url: self.fork_url.clone(),
			config: self.deployer_config.clone(),
			jsonl_prefix: self.jsonl_prefix.clone(),
		})
	}
}
