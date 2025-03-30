use clap::Parser;
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use mcr_types::block_commitment::BlockCommitment;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct CheckPostconfirmation {
    #[clap(flatten)]
    pub args: CheckPostconfirmationArgs,
}

#[derive(Parser)]
pub struct CheckPostconfirmationArgs {
    /// Block height to check postconfirmation for
    #[clap(long)]
    height: u64,

    /// MCR contract address
    #[clap(long, required = true)]
    mcr_address: String,

    /// RPC URL (optional, defaults to http://localhost:8545)
    #[clap(long, default_value = "http://localhost:8545")]
    rpc_url: String,
}

impl CheckPostconfirmation {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let config = Config::new(
            self.args.mcr_address.clone(),
            self.args.rpc_url.clone(),
            self.args.rpc_url.replace("http", "ws"),
            1,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            }),
            false,
            100000,
            3,
        );

        let client = config.build().await?;

        println!("Checking accepted commitment at height {}", self.args.height);
        let commitment = client.get_commitment_at_height(
            self.args.height
        ).await?;
        
        match commitment {
            None => println!("No accepted commitment found"),
            Some(commitment) => {
                println!("Found accepted commitment:");
                println!("{}", commitment);
            }
        }

        Ok(())
    }
} 