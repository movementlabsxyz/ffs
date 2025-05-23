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
use alloy::signers::local::PrivateKeySigner;
// use alloy::signers::Signer as _;
use alloy_network::EthereumWallet;
use alloy_primitives::Address;
use alloy_provider::fillers::BlobGasFiller;
use alloy_transport_ws::WsConnect;
use anyhow::Context;
use clap::Parser;
// use secure_signer::cryptography::secp256k1::Secp256k1;
use secure_signer::key::TryFromCanonicalString;
// use secure_signer_eth::Signer;
use mcr_protocol_client_core_util::{McrClientError, McrConfigOperations, McrViewOperations};
use secure_signer_loader::{identifiers::local::Local, identifiers::SignerIdentifier};
use serde::{Deserialize, Serialize};
// use tracing::info;

pub type StandardRpcProvider = FillProvider<
	JoinFill<
		JoinFill<
			alloy::providers::Identity,
			JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
		>,
		WalletFiller<EthereumWallet>,
	>,
	RootProvider,
>;

pub type StandardWsProvider = FillProvider<
	JoinFill<
		alloy::providers::Identity,
		JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
	>,
	RootProvider,
>;

pub type StandardClient = Client<StandardRpcProvider, StandardWsProvider>;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct Config {
	/// The view config
	#[clap(flatten)]
	pub view_config: ViewConfig,
	/// The signer identifier.
	#[arg(value_parser = SignerIdentifier::try_from_canonical_string)]
	#[arg(long)]
	pub signer_identifier: SignerIdentifier,
}

impl Config {
	// Helper to build the MCR client with a custom provider.
	async fn build_with_provider<R, W>(
		run_commitment_admin_mode: bool,
		rpc_provider: R,
		ws_provider: W,
		signer_address: Address,
		contract_address: Address,
		move_token_address: Address,
		_staking_address: Address,
		gas_limit: u64,
		send_transaction_retries: u32,
	) -> Result<Client<R, W>, anyhow::Error>
	where
		R: Provider + Clone,
		W: Provider + Clone,
	{
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
			move_token_address,
			// staking_address,
			send_transaction_error_rules,
			gas_limit,
			send_transaction_retries,
		})
	}
}

impl McrConfigOperations for Config {
	type Client = StandardClient;

	/// Builds the MCR client.
	async fn build(&self) -> Result<Self::Client, McrClientError> {
		let raw_key = self.signer_identifier.try_raw_private_key().context("failed to get the raw private key from the signer identifier; only local signers are currently supported").map_err(|e| McrClientError::Internal(e.into()))?;
		// add the 0x
		let raw_key_string = format!("0x{}", hex::encode(raw_key));
		let private_key_signer: PrivateKeySigner = raw_key_string
			.parse()
			.context("failed to parse the private key for the MCR settlement client")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let signer_address = private_key_signer.address();

		/*let signer_identifier: Box<dyn Load<Secp256k1> + Send> =
			Box::new(self.signer_identifier.clone());
		let signer_provider = signer_identifier.load().await?;

		let signer = Signer::try_new(signer_provider, Some(self.chain_id)).await?;*

		let signer_address = signer.address();
		info!("Signer address: {}", signer_address);*/
		let contract_address = self
			.view_config
			.mcr_contract_address
			.parse()
			.context("failed to parse the contract address for the MCR settlement client")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		// Build the rpc provider
		let rpc_provider = ProviderBuilder::new()
			.wallet(EthereumWallet::from(private_key_signer))
			.connect(&self.view_config.rpc_url)
			.await
			.context("failed to create the RPC provider for the MCR settlement client")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		// Build the ws provider
		let ws = WsConnect::new(self.view_config.ws_url.clone());
		let ws_provider = ProviderBuilder::new()
			.on_ws(ws)
			.await
			.context("failed to create the WebSocket provider for the MCR settlement client")
			.map_err(|e| McrClientError::Internal(e.into()))?;

		let client = Self::build_with_provider(
			self.view_config.run_commitment_admin_mode,
			rpc_provider,
			ws_provider,
			signer_address,
			contract_address,
			self.view_config
				.move_token_address
				.parse()
				.context("failed to parse the move token address for the MCR settlement client")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			self.view_config
				.staking_address
				.parse()
				.context("failed to parse the staking address for the MCR settlement client")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			self.view_config.gas_limit,
			self.view_config.transaction_send_retries,
		)
		.await
		.context(
			"Failed to create the MCR settlement client with the RPC provider and contract address",
		)
		.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(client)
	}
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct ViewConfig {
	/// The address of the MCR settlement contract.
	#[arg(long)]
	pub mcr_contract_address: String,
	/// The Ethereum RPC connection URL.
	#[arg(long)]
	pub rpc_url: String,
	/// The Ethereum WebSocket connection URL.
	#[arg(long)]
	pub ws_url: String,
	/// The Ethereum chain ID.
	#[arg(long)]
	pub chain_id: u64,
	/// Whether to run in settlement admin mode.
	#[arg(long)]
	pub run_commitment_admin_mode: bool,
	/// The gas limit for transactions.
	#[arg(long)]
	pub gas_limit: u64,
	/// The number of retries for sending transactions.
	#[arg(long)]
	pub transaction_send_retries: u32,
	/// The MCR address.
	#[arg(long)]
	pub mcr_address: String,
	/// The move token address.
	#[arg(long)]
	pub move_token_address: String,
	/// The staking address.
	#[arg(long)]
	pub staking_address: String,
}

impl McrViewOperations for ViewConfig {
	type Config = Config;

	fn try_into_config(self) -> Result<Self::Config, McrClientError> {
		// All zeros signer identifier
		let signer_identifier = SignerIdentifier::Local(Local {
			private_key_hex_bytes:
				"0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
		});

		Ok(Config { view_config: self, signer_identifier })
	}
}
