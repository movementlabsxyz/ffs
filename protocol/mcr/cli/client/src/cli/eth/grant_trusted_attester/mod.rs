use clap::Parser;
use mcr_protocol_client_eth_core::client::Client;
use secure_signer_loader::identifiers::{SignerIdentifier, local::Local};
use mcr_protocol_client_eth_core::config::Config;
use mcr_protocol_client_core_util::McrClientOperations;

#[derive(Debug, Parser, Clone)]
pub struct GrantTrustedAttester {
    /// The address to grant TRUSTED_ATTESTER role to
    #[clap(long)]
    pub attester: String,

    /// The MCR contract address
    #[clap(long)]
    pub mcr_address: String,

    /// The private key to use for signing transactions
    #[clap(long)]
    pub private_key: String,
}

impl GrantTrustedAttester {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let config = Config::new(
            self.mcr_address.clone(),
            "http://localhost:8545".to_string(),
            "ws://localhost:8545".to_string(),
            31337,
            SignerIdentifier::Local(Local {
                private_key_hex_bytes: self.private_key.clone(),
            }),
            false,
            1_000_000_000_000_000_000,
            3,
            self.mcr_address.clone(),
            16,
            "0x0000000000000000000000000000000000000000".to_string(),
            "0x0000000000000000000000000000000000000000".to_string(),
        );

        let client = config.build().await?;
        println!("Granting TRUSTED_ATTESTER role to {}", self.attester);
        client.grant_trusted_attester(self.attester.clone()).await.map_err(|e| anyhow::anyhow!(e))
    }
} 