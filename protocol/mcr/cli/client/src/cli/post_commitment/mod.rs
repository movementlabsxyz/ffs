use clap::Parser;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};
use mcr_protocol_client_eth_core::config::Config;

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct PostCommitment {
    #[clap(flatten)]
    pub args: PostCommitmentArgs,
}

#[derive(Parser)]
pub struct PostCommitmentArgs {
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
        let commitment = self.create_commitment()?;
        
        // Strip '0x' prefix if present
        let private_key = self.args.private_key.strip_prefix("0x")
            .unwrap_or(&self.args.private_key)
            .to_string();

        let config = Config::new(
            self.args.mcr_address.clone(),
            self.args.rpc_url.clone(),
            self.args.rpc_url.replace("http", "ws"),
            1,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: private_key,
            }),
            false,
            100000,
            3,
            self.args.mcr_address.clone(),
            16,
            "0x0000000000000000000000000000000000000000".to_string(),
            "0x0000000000000000000000000000000000000000".to_string(),
        );

        let client = config.build().await?;
        
        println!("Posting commitment to MCR contract...");
        // TODO: Call client.post_block_commitment() with the commitment
        
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