use clap::Parser;
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};

#[derive(Parser)]
pub struct GetStake {
    /// Private key for signing transactions (optional)
    #[clap(long, default_value = "0x1111111111111111111111111111111111111111111111111111111111111111")]
    pub private_key: String,

    /// RPC URL (optional, defaults to http://localhost:8545)
    #[clap(long, default_value = "http://localhost:8545")]
    pub rpc_url: String,

    /// The attester address
    #[clap(long)]
    pub attester: String,

    /// The custodian (MOVE token) address
    #[clap(long)]
    pub custodian: String,

    /// The MCR contract address
    #[clap(long)]
    pub mcr_address: String,
}

impl GetStake {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        // Strip '0x' prefix if present
        let private_key = self.private_key.strip_prefix("0x")
            .unwrap_or(&self.private_key)
            .to_string();

        let config = Config::new(
            self.mcr_address.clone(),
            self.rpc_url.clone(),
            self.rpc_url.replace("http", "ws"),
            1,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: private_key,
            }),
            false,
            1_000_000,
            3,
            self.mcr_address.clone(),
            16,
            self.custodian.clone(),
            self.mcr_address.clone(),
        );
        let client = config.build()
            .await?;

        let stake = client.get_stake(self.custodian.clone(), self.attester.clone()).await?;
        println!("{}", stake);
        Ok(())
    }
} 