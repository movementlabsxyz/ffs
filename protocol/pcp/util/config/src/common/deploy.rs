use alloy::signers::local::PrivateKeySigner;
use ffs_environment::env_short_default;
use secure_signer::key::TryFromCanonicalString;
use secure_signer_loader::identifiers::{local::Local, SignerIdentifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	#[serde(default = "pcp_deployment_working_directory")]
	pub pcp_deployment_working_directory: String,
	#[serde(default = "default_signer_identifier")]
	pub signer_identifier: SignerIdentifier,
}

env_short_default!(
	pcp_deployment_working_directory,
	String,
	"protocol-units/settlement/pcp/contracts"
);

env_short_default!(
	pcp_local_anvil_account_private_key,
	String,
	PrivateKeySigner::random().to_bytes().to_string()
);

pub fn default_signer_identifier() -> SignerIdentifier {
	match std::env::var("PCP_SIGNER_IDENTIFIER") {
		Ok(str_value) => SignerIdentifier::try_from_canonical_string(&str_value).unwrap(),
		Err(_) => SignerIdentifier::Local(Local {
			// todo: validate this is a valid private key
			private_key_hex_bytes: pcp_local_anvil_account_private_key(),
		}),
	}
}

pub fn maybe_deploy() -> Option<Config> {
	match std::env::var("MAYBE_DEPLOY_PCP") {
		Ok(str_value) => {
			// if it parses as true then we want to deploy under the default config
			let bool_value = str_value.parse::<bool>().unwrap_or(false);

			if bool_value {
				Some(Config::default())
			} else {
				None
			}
		}
		Err(_) => None,
	}
}

impl Default for Config {
	fn default() -> Self {
		Config {
			pcp_deployment_working_directory: pcp_deployment_working_directory(),
			signer_identifier: default_signer_identifier(),
		}
	}
}
