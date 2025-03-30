use clap::Parser;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Stake {
    #[clap(flatten)]
    pub args: StakeArgs,
}

#[derive(Parser)]
pub struct StakeArgs {
    /// Amount of MOVE tokens to stake
    #[clap(long, required = true)]
    amount: f64,

    /// Private key for signing transactions
    #[clap(long, required = true)]
    private_key: String,

    /// MCR contract address
    #[clap(long, required = true)]
    mcr_address: String,

    /// RPC URL (optional, defaults to http://localhost:8545)
    #[clap(long, default_value = "http://localhost:8545")]
    rpc_url: String,

    /// Move token address
    #[clap(long)]
    move_token_address: String,

    /// Staking address
    #[clap(long)]
    staking_address: String,
}

impl Stake {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
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
            16,  // block_lead_tolerance
            self.args.move_token_address.clone(),
            self.args.staking_address.clone(),
        );

        let client = config.build().await?;
        
        // Convert amount from float to uint256
        let amount = (self.args.amount * 100_000_000.0) as u64;
        
        println!("Staking {} MOVE tokens...", self.args.amount);
        client.stake(amount).await?;
        
        Ok(())
    }
} 