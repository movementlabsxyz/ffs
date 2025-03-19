pub mod eth;
use clap::{Parser, Subcommand};
use pcp_protocol_client_core_eth::config::Config;
use pcp_protocol_client_core_util::PcpClientOperations;
use pcp_types::block_commitment::{SuperBlockCommitment, Commitment, Id};
use sha3::{Digest, Keccak256};
use secure_signer_loader::identifiers::SignerIdentifier;
use secure_signer_loader::identifiers::local::Local;

/// The `pcp-protocol-client` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct PcpProtocolClient {
	#[clap(subcommand)]
	command: Option<PcpProtocolClientSubcommand>,
}

/// The subcommands of the `pcp-protocol-client` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum PcpProtocolClientSubcommand {
	Run,
	#[clap(subcommand)]
	Eth(eth::Eth),
	/// Post a commitment to a PCP implementation
	PostCommitment(PostCommitmentArgs),
}

#[derive(clap::Args)]
pub struct PostCommitmentArgs {
	/// Hex-encoded commitment
	#[clap(long, conflicts_with = "preimage_string", required_unless_present = "preimage_string")]
	commitment_hex: Option<String>,

	/// String to be hashed into a commitment
	#[clap(long, conflicts_with = "commitment_hex", required_unless_present = "commitment_hex")]
	preimage_string: Option<String>,
}

/// Implement the `From` trait for `PcpProtocolClient` to convert it into a `PcpProtocolClientSubcommand`.
impl From<PcpProtocolClient> for PcpProtocolClientSubcommand {
	fn from(client: PcpProtocolClient) -> Self {
		client.command.unwrap_or(PcpProtocolClientSubcommand::Run)
	}
}

/// Implement the `PcpProtocolClient` CLI.
impl PcpProtocolClient {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: PcpProtocolClientSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `PcpProtocolClientSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl PcpProtocolClientSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			PcpProtocolClientSubcommand::Run => {
				println!("pcp-protocol-client is under development. Please check back later.");
			}
			PcpProtocolClientSubcommand::Eth(eth) => eth.execute().await?,
			PcpProtocolClientSubcommand::PostCommitment(args) => {
				self.handle_post_commitment(args).await?;
			}
		}
		Ok(())
	}

	/// Handle the post commitment command.
	async fn handle_post_commitment(&self, args: &PostCommitmentArgs) -> Result<(), anyhow::Error> {
		let commitment = self.create_commitment(args)?;
		
		// Get config and post commitment
		let config = get_config()?;
		println!("Config: {:?}", config);
		let client = config.build().await?;
		println!("Starting post commitment process...");
		client.post_block_commitment(commitment).await?;
		println!("Successfully posted commitment");
		
		Ok(())
	}

	/// Create a commitment from the given arguments.
	fn create_commitment(&self, args: &PostCommitmentArgs) -> Result<SuperBlockCommitment, anyhow::Error> {
		if let Some(hex) = &args.commitment_hex {
			// Parse hex commitment
			let bytes = hex::decode(hex)?;
			let bytes_len = bytes.len();
			Ok(SuperBlockCommitment::new(
				0, // height
				Id::new([0; 32]), // block id
				Commitment::new(bytes.try_into()
					.map_err(|_| anyhow::anyhow!(
						"Invalid commitment length. Expected 32 bytes (64 hex characters), got {} bytes ({} hex characters)",
						bytes_len,
						bytes_len * 2
					))?)
			))
		} else if let Some(preimage) = &args.preimage_string {
			// Hash preimage to get commitment
			let mut hasher = Keccak256::new();
			hasher.update(preimage.as_bytes());
			let result = hasher.finalize();
			Ok(SuperBlockCommitment::new(
				0, // height
				Id::new([0; 32]), // block id
				Commitment::new(result.into()),
			))
		} else {
			unreachable!("clap ensures one option is present")
		}
	}
}

/// Get the config for the PCP client.
fn get_config() -> Result<Config, anyhow::Error> {
	let config = Config::new(
		"0x1234567890123456789012345678901234567890".to_string(),  // PCP contract address
		"http://localhost:8545".to_string(),  // RPC URL
		"ws://localhost:8546".to_string(),  // WS URL
		1,  // Chain ID
		SignerIdentifier::Local(Local {
			private_key_hex_bytes: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
		}),
		false,  // Run commitment admin mode
		100000,  // Gas limit
		3,  // Transaction send retries
	);
	Ok(config)
}
