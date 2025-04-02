use clap::Parser;
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_types::block_commitment::BlockCommitment;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct GetBlockCommitment {
    #[clap(flatten)]
    pub args: GetBlockCommitmentArgs,
}

#[derive(Parser)]
pub struct GetBlockCommitmentArgs {
    /// Block height to check commitment for
    #[clap(long)]
    height: u64,

    /// Attester address to check commitment for
    #[clap(long, required = true)]
    attester: String,

    /// MCR contract address
    #[clap(long, required = true)]
    mcr_address: String,

    /// RPC URL (optional, defaults to http://localhost:8545)
    #[clap(long, default_value = "http://localhost:8545")]
    rpc_url: String,

    /// Private key for signing transactions (optional)
    #[clap(long)]
    private_key: Option<String>,
}

impl GetBlockCommitment {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        // Use provided key or fallback to dummy key
        let private_key = self.args.private_key
            .as_ref()
            .map(|k| k.strip_prefix("0x").unwrap_or(k))
            .unwrap_or("0000000000000000000000000000000000000000000000000000000000000000")
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
            10_000_000,  // Increased gas limit to match other commands
            3,
            self.args.mcr_address.clone(),
            16,
            "0x0000000000000000000000000000000000000000".to_string(),
            "0x0000000000000000000000000000000000000000".to_string(),
        );

        let client = config.build().await?;

        println!("Checking commitment for attester {} at height {}", 
            self.args.attester, self.args.height);
        
        let commitment = client.get_validator_commitment_at_height(
            self.args.height,
            self.args.attester.clone(),
        ).await?;
        
        match commitment {
            None => println!("No commitment found"),
            Some(commitment) => {
                println!("Found commitment:");
                println!("{}", commitment);
            }
        }

        Ok(())
    }
} 