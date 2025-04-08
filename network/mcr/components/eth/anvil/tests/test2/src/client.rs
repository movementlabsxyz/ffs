use mcr_protocol_client_core_util::{McrClientOperations, U256};
use mcr_protocol_client_eth_core::config::StandardClient;
use mcr_types::commitment::Commitment;

use alloy_primitives::{Address, TxKind, keccak256};
use alloy_provider::{ProviderBuilder, Provider};
use alloy_rpc_types::{TransactionRequest, TransactionInput};
use alloy::signers::local::PrivateKeySigner;
use alloy_consensus::transaction::SignableTransaction;
use bytes::BytesMut;
use alloy_network::TxSigner;
use url::Url;
use hex;
use alloy_primitives::Bytes;
use std::borrow::Cow;
// use alloy_rpc_types_eth::request::CallRequest;

#[derive(Debug, Clone)]
pub enum Act {
    PostCommitment(Commitment),
    GetTokenBalance {
        token_address: String,
        address: String,
    },
    TransferTokens {
        token_address: String,
        to: String,
        amount: U256,
        private_key_sender: String,
        address_sender: String,
    },
}

pub struct Client {
    pub mcr_protocol_client: StandardClient,
}

impl Client {
    pub fn new(mcr_protocol_client: StandardClient) -> Self {
        Self { mcr_protocol_client }
    }

    pub async fn act(&self, act: Act) -> Result<Option<U256>, anyhow::Error> {
        match act {
            Act::PostCommitment(commitment) => {
                self.handle_post_commitment(commitment).await?;
                Ok(None)
            },
            Act::GetTokenBalance { token_address, address } => {
                let balance = self.handle_get_token_balance(&token_address, &address).await?;
                Ok(Some(balance))
            }
            Act::TransferTokens {
                token_address,
                to,
                amount,
                private_key_sender,
                address_sender,
            } => {
                self.handle_transfer_tokens(token_address, to, amount, private_key_sender, address_sender).await?;
                Ok(None)
            }
        }
    }

    // post a commitment
    async fn handle_post_commitment(&self, commitment: Commitment) -> Result<(), anyhow::Error> {
        self.mcr_protocol_client.post_commitment(commitment).await?;
        Ok(())
    }

    async fn handle_get_token_balance(
        &self,
        token_address: &str,
        address: &str,
    ) -> Result<U256, anyhow::Error> {
        use serde_json::json;

        let url = Url::parse("http://localhost:8545")?;
        let provider = ProviderBuilder::new().on_http(url);

        let token: Address = token_address.parse()?;
        let owner: Address = address.parse()?;

        // Encode selector + address
        let selector = &keccak256(b"balanceOf(address)")[..4];
        let mut data = Vec::from(selector);
        data.extend_from_slice(&[0u8; 12]); // left-pad address
        data.extend_from_slice(&owner.to_vec());
        let data_hex = format!("0x{}", hex::encode(data));

        // Build eth_call params manually
        let params = json!([{
            "to": format!("{:#x}", token),
            "data": data_hex
        }, "latest"]);

        // Send raw RPC call
        let raw: String = provider.raw_request(Cow::Borrowed("eth_call"), params).await?;
        let raw_bytes = hex::decode(raw.strip_prefix("0x").unwrap_or(&raw))?;
        let balance = U256::from_be_bytes::<32>(
            raw_bytes
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("Expected 32 bytes for U256"))?,
        );
        println!("[handle_get_token_balance] token balance: {}", balance);
        Ok(balance)
    }


    // transfer move tokens
    async fn handle_transfer_tokens(
        &self,
        token_address: String,
        to: String,
        amount: U256,
        private_key_sender: String,
        address_sender: String,
    ) -> Result<(), anyhow::Error> {
        println!("[handle_transfer_tokens] token_address: {:?}", token_address);
        println!("[handle_transfer_tokens] to: {:?}", to);
        println!("[handle_transfer_tokens] amount: {:?}", amount);
        println!("[handle_transfer_tokens] private_key_sender: {:?}", private_key_sender);
        println!("[handle_transfer_tokens] address_sender: {:?}", address_sender);

        // get the balance of the token using handle_get_token_balance
        let url = Url::parse("http://localhost:8545")?;
        let provider = ProviderBuilder::new().on_http(url);
        let token: Address = token_address.parse()?;
        let balance = self.handle_get_token_balance(&token_address, &address_sender).await?;
        println!("[handle_transfer_tokens] balance: {:?}", balance);
        // if the balance is 0, then there is an error. exit.
        if balance == U256::ZERO {
            return Err(anyhow::anyhow!("balance is 0"));
        }

        let url = Url::parse("http://localhost:8545")?;
        let provider = ProviderBuilder::new().on_http(url);
    
        let token: Address = token_address.parse()?;
        let to_addr: Address = to.parse()?;
    
        // Encode ERC20 transfer(address,uint256)
        let selector = keccak256(b"transfer(address,uint256)")[..4].to_vec();
        let mut data = selector;
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(&to_addr.to_vec());
        data.extend_from_slice(&amount.to_be_bytes::<32>());
    
        // Use alloy_signers instead of raw k256
        let key_bytes = hex::decode(private_key_sender.strip_prefix("0x").unwrap_or(&private_key_sender))?;
        let key_array: [u8; 32] = key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("private key must be 32 bytes"))?;
        let signer = PrivateKeySigner::from_bytes((&key_array).into())?;

        // let signer = PrivateKeySigner::from_bytes(&key_array)?;
        let from = signer.address();
    
        // create the transaction request
        let tx = TransactionRequest {
            from: Some(from),
            to: Some(TxKind::Call(token)),
            input: TransactionInput::new(data.into()),
            gas: Some(100_000),
            nonce: Some(provider.get_transaction_count(from).await?),
            chain_id: Some(provider.get_chain_id().await?.into()),
            gas_price: Some(provider.get_gas_price().await?.into()),
            ..Default::default()
        };

        // build the typed transaction
        let typed_tx = tx
            .build_typed_tx()
            .map_err(|e| anyhow::anyhow!("failed to build typed transaction: {:?}", e))?;

        // sign the transaction
        let mut tx = typed_tx;
        let sig = signer.sign_transaction(&mut tx).await?;
        let signed = tx.into_signed(sig);
    
        let mut out = BytesMut::new();
        signed.rlp_encode(&mut out);
        let raw_tx = out.to_vec();
    
        let tx_hash = provider.send_raw_transaction(&raw_tx).await?;
        println!("Sent tx: {:?}", tx_hash);
    
        Ok(())
    }



}
