use crate::client::Client;
use crate::util::send_eth_transaction::InsufficentFunds;
use crate::util::send_eth_transaction::SendTransactionErrorRule;
use crate::util::send_eth_transaction::UnderPriced;
use crate::util::send_eth_transaction::VerifyRule;
use alloy::providers::fillers::ChainIdFiller;
use alloy::providers::fillers::FillProvider;
use alloy::providers::fillers::GasFiller;
use alloy::providers::fillers::JoinFill;
use alloy::providers::fillers::NonceFiller;
use alloy::providers::fillers::WalletFiller;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::signers::Signer as _;
use alloy_network::Ethereum;
use alloy_network::EthereumWallet;
use alloy_primitives::Address;
use alloy_transport::BoxTransport;
use alloy_transport_ws::WsConnect;
use anyhow::Context;
use clap::Parser;
use secure_signer::cryptography::secp256k1::Secp256k1;
use secure_signer::key::TryFromCanonicalString;
use secure_signer_eth::Signer;
use secure_signer_loader::{identifiers::SignerIdentifier, Load};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	/// The address of the PCP settlement contract.
	pub pcp_contract_address: String,
	/// The Ethereum RPC connection URL.
	pub rpc_url: String,
	/// The Ethereum WebSocket connection URL.
	pub ws_url: String,
	/// The Ethereum chain ID.
	pub chain_id: u64,
	/// The signer identifier.
	#[arg(value_parser = SignerIdentifier::try_from_canonical_string)]
	pub signer_identifier: SignerIdentifier,
	/// Whether to run in settlement admin mode.
	pub run_commitment_admin_mode: bool,
	/// The gas limit for transactions.
	pub gas_limit: u64,
	/// The number of retries for sending transactions.
	pub transaction_send_retries: u32,
}

impl Config {
	/// Creates a new PCP client configuration.
	pub fn new(
		pcp_contract_address: String,
		rpc_url: String,
		ws_url: String,
		chain_id: u64,
		signer_identifier: SignerIdentifier,
		run_commitment_admin_mode: bool,
		gas_limit: u64,
		transaction_send_retries: u32,
	) -> Self {
		Config {
			pcp_contract_address,
			rpc_url,
			ws_url,
			chain_id,
			signer_identifier,
			run_commitment_admin_mode,
			gas_limit,
			transaction_send_retries,
		}
	}

	/// Builds the PCP client.
	pub async fn build(
		self,
	) -> Result<
		Client<
			FillProvider<
				JoinFill<
					JoinFill<
						JoinFill<JoinFill<alloy::providers::Identity, GasFiller>, NonceFiller>,
						ChainIdFiller,
					>,
					WalletFiller<EthereumWallet>,
				>,
				RootProvider<BoxTransport>,
				BoxTransport,
				Ethereum,
			>,
		>,
		anyhow::Error,
	> {
		let signer_identifier: Box<dyn Load<Secp256k1> + Send> =
			Box::new(self.signer_identifier.clone());
		let signer_provider = signer_identifier.load().await?;
		let signer = Signer::try_new(signer_provider, Some(self.chain_id)).await?;

		let signer_address = signer.address();
		info!("Signer address: {}", signer_address);
		let contract_address = self
			.pcp_contract_address
			.parse()
			.context("Failed to parse the contract address for the PCP settlement client")?;

		// Build the rpc provider
		let rpc_provider = ProviderBuilder::new()
			.with_recommended_fillers()
			.wallet(EthereumWallet::from(signer))
			.on_builtin(&self.rpc_url)
			.await
			.context("Failed to create the RPC provider for the PCP settlement client")?;

		let client = Self::build_with_provider(
			self.run_commitment_admin_mode,
			rpc_provider,
			self.ws_url,
			signer_address,
			contract_address,
			self.gas_limit,
			self.transaction_send_retries,
		)
		.await
		.context(
			"Failed to create the PCP settlement client with the RPC provider and contract address",
		)?;
		Ok(client)
	}

	// Helper to build the PCP client with a custom provider.
	async fn build_with_provider<S, P>(
		run_commitment_admin_mode: bool,
		rpc_provider: P,
		ws_url: S,
		signer_address: Address,
		contract_address: Address,
		gas_limit: u64,
		send_transaction_retries: u32,
	) -> Result<Client<P>, anyhow::Error>
	where
		P: Provider + Clone,
		S: Into<String>,
	{
		let ws = WsConnect::new(ws_url);

		let ws_provider = ProviderBuilder::new()
			.on_ws(ws)
			.await
			.context("Failed to create the WebSocket provider for the PCP settlement client")?;

		let rule1: Box<dyn VerifyRule> = Box::new(SendTransactionErrorRule::<UnderPriced>::new());
		let rule2: Box<dyn VerifyRule> =
			Box::new(SendTransactionErrorRule::<InsufficentFunds>::new());
		let send_transaction_error_rules = vec![rule1, rule2];

		Ok(Client {
			run_commitment_admin_mode,
			rpc_provider,
			ws_provider,
			signer_address,
			contract_address,
			send_transaction_error_rules,
			gas_limit,
			send_transaction_retries,
		})
	}
}
