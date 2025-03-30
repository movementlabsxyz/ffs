use clap::Parser;
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_types::block_commitment::BlockCommitment;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct CheckCommitment {
    #[clap(flatten)]
    pub args: CheckCommitmentArgs,
}

#[derive(Parser)]
pub struct CheckCommitmentArgs {
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
}

impl CheckCommitment {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let config = Config::new(
            self.args.mcr_address.clone(),
            self.args.rpc_url.clone(),
            self.args.rpc_url.replace("http", "ws"),
            1,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
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