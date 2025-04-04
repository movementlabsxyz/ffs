use clap::Parser;
use pcp_protocol_client_core_eth::config::Config;
use pcp_protocol_client_core_util::PcpClientOperations;
use pcp_types::commitment::{Commitment, CommitmentValue, CommitmentId};
use secure_signer_loader::identifiers::local::Local;
use secure_signer_loader::identifiers::SignerIdentifier;
use sha3::{Digest, Keccak256};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct PostCommitment {
	#[clap(flatten)]
	pub args: PostCommitmentArgs,
}

#[derive(Parser)]
pub struct PostCommitmentArgs {
	/// Hex-encoded commitment
	#[clap(long, conflicts_with = "preimage_string", required_unless_present = "preimage_string")]
	// for option `--commitment-hex "0x1234567890abcdef"`
	commitment_hex: Option<String>,

	/// String to be hashed into a commitment
	#[clap(long, conflicts_with = "commitment_hex", required_unless_present = "commitment_hex")]
	// for option `--preimage-string "test message"`
	preimage_string: Option<String>,
}

impl PostCommitment {
	/// Handle the post commitment command.
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let commitment = self.create_commitment()?;

		// Get config and post commitment
		let config = get_config()?;
		println!("Config: {:?}", config);
		let client = config.build().await?;
		println!("Starting post commitment process...");
		client.post_commitment(commitment).await?;
		println!("Successfully posted commitment");

		Ok(())
	}

	/// Create a commitment from the given arguments.
	fn create_commitment(&self) -> Result<Commitment, anyhow::Error> {
		if let Some(hex) = &self.args.commitment_hex {
			// Parse hex commitment
			let bytes = hex::decode(hex)?;
			let bytes_len = bytes.len();
			Ok(Commitment::new(
                0, // height
                CommitmentId::new([0; 32]), // block id
                CommitmentValue::new(bytes.try_into()
                    .map_err(|_| anyhow::anyhow!(
                        "Invalid commitment length. Expected 32 bytes (64 hex characters), got {} bytes ({} hex characters)",
                        bytes_len,
                        bytes_len * 2
                    ))?)
            ))
		} else if let Some(preimage) = &self.args.preimage_string {
			// Hash preimage to get commitment
			let mut hasher = Keccak256::new();
			hasher.update(preimage.as_bytes());
			let result = hasher.finalize();
			Ok(Commitment::new(
				0,                // height
				CommitmentId::new([0; 32]), // block id
				CommitmentValue::new(result.into()),
			))
		} else {
			unreachable!("clap ensures one option is present")
		}
	}
}

/// Get the config for the PCP client.
fn get_config() -> Result<Config, anyhow::Error> {
	let config = Config::new(
		"0x1234567890123456789012345678901234567890".to_string(), // PCP contract address
		"http://localhost:8545".to_string(),                      // RPC URL
		"ws://localhost:8546".to_string(),                        // WS URL
		1,                                                        // Chain ID
		SignerIdentifier::Local(Local {
			private_key_hex_bytes:
				"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
		}),
		false,  // Run commitment admin mode
		100000, // Gas limit
		3,      // Transaction send retries
	);
	Ok(config)
}
