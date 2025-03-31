use clap::Parser;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_types::block_commitment::{BlockCommitment, Commitment, Id};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct PostCommitment {
    #[clap(flatten)]
    pub args: PostCommitmentArgs,
}

#[derive(Parser)]
pub struct PostCommitmentArgs {
    /// Block height to post commitment for
    #[clap(long)]
    height: u64,

    /// MCR contract address
    #[clap(long, required = true)]
    mcr_address: String,

    /// Hex-encoded commitment
    #[clap(long, conflicts_with = "preimage_string", required_unless_present = "preimage_string")]
    // for option `--commitment-hex "0x1234567890abcdef"`
    commitment_hex: Option<String>,

    /// String to be hashed into a commitment
    #[clap(long, conflicts_with = "commitment_hex", required_unless_present = "commitment_hex")]
    // for option `--preimage-string "test message"`
    preimage_string: Option<String>,

    /// Private key for signing transactions
    #[clap(long, required = true)]
    private_key: String,

    /// RPC URL (optional, defaults to http://localhost:8545)
    #[clap(long, default_value = "http://localhost:8545")]
    rpc_url: String,
}

impl PostCommitment {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        println!("Debug [post_commitment] - Creating commitment from input...");
        let commitment_bytes = self.create_commitment()?;
        println!("Debug [post_commitment] - Commitment bytes: 0x{}", hex::encode(&commitment_bytes));
        
        // Create a unique block ID by hashing height with commitment
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(&[0x01]); // Add prefix to make it different from commitment
        hasher.update(&commitment_bytes);
        hasher.update(&self.args.height.to_be_bytes());
        let block_id = hasher.finalize().into();
        
        let commitment = BlockCommitment::new(
            self.args.height,  // Use height from args
            Id::new(block_id),
            Commitment::new(commitment_bytes),
        );
        println!("Debug [post_commitment] - Block height: {}", commitment.height());
        println!("Debug [post_commitment] - Block ID: {:?}", commitment.block_id());
        println!("Debug [post_commitment] - Commitment: {:?}", commitment.commitment());
        
        // Strip '0x' prefix if present
        let private_key = self.args.private_key.strip_prefix("0x")
            .unwrap_or(&self.args.private_key)
            .to_string();
        println!("Debug [post_commitment] - Using MCR address: {}", self.args.mcr_address);

        let config = Config::new(
            self.args.mcr_address.clone(),
            self.args.rpc_url.clone(),
            self.args.rpc_url.replace("http", "ws"),
            1,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: private_key,
            }),
            false,
            1_000_000_000_000_000_000,
            3,
            self.args.mcr_address.clone(),
            16,
            "0x0000000000000000000000000000000000000000".to_string(),
            "0x0000000000000000000000000000000000000000".to_string(),
        );

        let client = config.build().await?;
        
        println!("Posting commitment to MCR contract...");
        // Check block height tolerance
        let last_accepted = client.get_last_accepted_block_height().await?;
        let tolerance = client.get_leading_block_tolerance().await?;
        println!("Debug [post_commitment] - Last accepted block height: {}", last_accepted);
        println!("Debug [post_commitment] - Leading block tolerance: {}", tolerance);
        println!("Debug [post_commitment] - Our block height: {}", self.args.height);
        
        // Check if we already have a commitment at this height
        let existing = client.get_posted_commitment_at_height(self.args.height).await?;
        println!("Debug [post_commitment] - Existing commitment at height {}: {:?}", self.args.height, existing);
        if existing.is_some() {
            println!("Debug [post_commitment] - Commitment already exists at height {}, skipping...", self.args.height);
            return Ok(());
        }
        
        println!("Debug [post_commitment] - About to call post_block_commitment...");
        
        // Check if cast is available
        match std::process::Command::new("cast").arg("--version").output() {
            Ok(output) => println!("Debug [post_commitment] - Cast version: {}", String::from_utf8_lossy(&output.stdout)),
            Err(e) => println!("Debug [post_commitment] - Cast not found: {}", e),
        }
        
        client.post_block_commitment(commitment).await?;
        
        Ok(())
    }

    fn create_commitment(&self) -> Result<[u8; 32], anyhow::Error> {
        if let Some(hex) = &self.args.commitment_hex {
            // Parse hex commitment
            let bytes = hex::decode(hex)?;
            let bytes_len = bytes.len();
            bytes.try_into()
                .map_err(|_| anyhow::anyhow!(
                    "Invalid commitment length. Expected 32 bytes (64 hex characters), got {} bytes ({} hex characters)",
                    bytes_len,
                    bytes_len * 2
                ))
        } else if let Some(preimage) = &self.args.preimage_string {
            // Hash preimage to get commitment
            use sha3::{Digest, Keccak256};
            let mut hasher = Keccak256::new();
            hasher.update(preimage.as_bytes());
            Ok(hasher.finalize().into())
        } else {
            unreachable!("clap ensures one option is present")
        }
    }
} 