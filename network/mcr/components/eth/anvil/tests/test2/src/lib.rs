mod client;
pub mod config;

pub use client::{Act, Client};
pub use config::Config;

use anyhow::Context;
use mcr_network_anvil_component_core::dev::lifecycle::up::Up;
use mcr_protocol_client_eth_core::config::Config as EthConfig;
use mcr_protocol_deployer_eth_core::artifacts::output::Artifacts;
use network_anvil_component_core::util::parser::AnvilData;
use secure_signer::key::TryFromCanonicalString;
use secure_signer_loader::identifiers::SignerIdentifier;
use tokio::time::Duration;
use mcr_protocol_client_core_util::U256;


pub struct Test {
    up: Up,
}

pub struct UpState {
    pub anvil_data: AnvilData,
    pub artifacts: Artifacts,
}

impl UpState {
    pub fn try_to_default_mcr_protocol_client_config(&self) -> Result<EthConfig, anyhow::Error> {
        let rpc_url = "http://localhost:8545".to_string();
        let ws_url = "ws://localhost:8545".to_string();
        let chain_id = self.anvil_data.chain_id;
        let commitment_lead_tolerance = 100;

        let signer_identifier_hex_key = self.anvil_data.private_keys[0].clone();
        let canonical_identifier_string = format!(
            "local::{}",
            signer_identifier_hex_key
                .strip_prefix("0x")
                .context("invalid signer identifier")?
        );
        let signer_identifier =
            SignerIdentifier::try_from_canonical_string(&canonical_identifier_string)
                .map_err(|_| anyhow::anyhow!("invalid signer identifier"))?;

        Ok(EthConfig {
            mcr_contract_address: self.artifacts.mcr_proxy.clone(),
            rpc_url: rpc_url.clone(),
            ws_url: ws_url.clone(),
            chain_id: chain_id,
            signer_identifier: signer_identifier,
            run_commitment_admin_mode: false,
            gas_limit: 323924465909782,
            transaction_send_retries: 3,
            mcr_address: self.artifacts.mcr_proxy.clone(),
            commitment_lead_tolerance: commitment_lead_tolerance,
            move_token_address: self.artifacts.token_proxy.clone(),
            staking_address: self.artifacts.staking_proxy.clone(),
        })
    }

    pub async fn try_build_default_mcr_protocol_client(&self) -> Result<Client, anyhow::Error> {
        let mcr_protocol_client_config = self.try_to_default_mcr_protocol_client_config()?;
        let mcr_protocol_client = mcr_protocol_client_config.build().await?;
        Ok(Client::new(mcr_protocol_client))
    }
}

impl Test {
    pub fn new(config: Config) -> Self {
        Test { up: Up::new(config.up) }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let anvil_data = self.up.anvil_data().clone();
        let artifacts = self.up.artifacts().clone();

        let up_task = kestrel::task(async move { self.up.run().await });

        println!("waiting for anvil data");
        let anvil_data = anvil_data.read().wait_for(Duration::from_secs(30)).await?;
        println!("waiting for artifacts");
        let artifacts = artifacts.read().wait_for(Duration::from_secs(30)).await?;
        println!("up state");
        let up_state = UpState { anvil_data, artifacts };

        let mcr_protocol_client = up_state.try_build_default_mcr_protocol_client().await?;

        // Get the second account from anvil data to transfer to
        // print all fields of anvil data
        println!("------------------------------------------------------");
        println!("anvil data: {:?}", up_state.anvil_data);
        println!("------------------------------------------------------");
        println!("artifacts: {:?}", up_state.artifacts);
        println!("------------------------------------------------------");

        // println!("transferring tokens to {:?}", up_state.anvil_data.addresses[1]);

        let recipient = up_state.anvil_data.signers[1].clone();
        let move_token = up_state.artifacts.token_proxy;

        println!("......................................................");
        println!("recipient: {:?}", recipient);
        println!("move token: {:?}", move_token);
        println!("transferring tokens to {:?}", recipient);
        println!("......................................................");

        // for each account, check the balance of the move token
        for i in 0..up_state.anvil_data.signers.len() {
            let balance = mcr_protocol_client.act(Act::GetTokenBalance {
                token_address: move_token.clone(),
                address: up_state.anvil_data.signers[i].clone(),
            }).await?;
            println!("balance of account {:?}: {:?}", up_state.anvil_data.signers[i], balance);
        }

        // Transfer 1000 tokens to the second account
        mcr_protocol_client.act(Act::TransferTokens {
            token_address: move_token,
            to: recipient,
            amount: U256::from(1000),
            private_key_sender: up_state.anvil_data.private_keys[1].clone(),
            address_sender: up_state.anvil_data.signers[1].clone(),
        }).await?;

        // // Post a commitment
        // mcr_protocol_client.act(Act::PostCommitment(Commitment::default())).await?;

        kestrel::end!(up_task)?;

        Ok(())
    }
}
