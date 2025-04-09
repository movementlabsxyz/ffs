use crate::client::{Client, PollingParameters};
use alloy::signers::local::PrivateKeySigner;
use alloy_network::EthereumWallet;
use anyhow::Context;
use clap::Parser;
use helios::common::types::BlockTag;
use helios::ethereum::{
	config::networks::Network, database::FileDB, EthereumClient, EthereumClientBuilder,
};
use mcr_protocol_client_core_util::{McrClientError, McrConfigOperations, McrViewOperations};
use mcr_protocol_client_eth_core::config::{
	Config as CoreConfig, StandardRpcProvider, StandardWsProvider, ViewConfig as CoreViewConfig,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

pub type StandardClient = Client<StandardRpcProvider, StandardWsProvider>;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub enum Finality {
	/// The light node will sync to the latest block.
	Latest,
	/// The light node will sync to the finalized block.
	Finalized,
}

impl FromStr for Finality {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"latest" => Self::Latest,
			"finalized" => Self::Finalized,
			other => return Err(anyhow::anyhow!("invalid finality: {}", other)),
		})
	}
}

impl From<Finality> for BlockTag {
	fn from(finality: Finality) -> Self {
		match finality {
			Finality::Latest => BlockTag::Latest,
			Finality::Finalized => BlockTag::Finalized,
		}
	}
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct LightNodeConfig {
	/// The directory to store the light node data.
	#[clap(long)]
	pub light_node_data_dir: String,

	/// The Ethereum network type to use for light node consensus parameterizaton.
	#[clap(long, default_value = "mainnet")]
	pub light_node_network: Network,

	/// The consensus RPC for the light node to use
	#[clap(long)]
	pub consensus_rpc_url: String,

	/// The finality configuration for the light node.
	#[clap(long)]
	pub finality: Finality,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct Config {
	#[clap(flatten)]
	pub core: CoreConfig,

	#[clap(flatten)]
	pub light_node_config: LightNodeConfig,
}

impl McrConfigOperations for Config {
	type Client = StandardClient;

	async fn build(&self) -> Result<StandardClient, McrClientError> {
		// build the signer
		let raw_key = self.core.signer_identifier.try_raw_private_key().context("failed to get the raw private key from the signer identifier; only local signers are currently supported").map_err(|e| McrClientError::Internal(e.into()))?;
		// add the 0x
		let raw_key_string = format!("0x{}", hex::encode(raw_key));
		let private_key_signer: PrivateKeySigner = raw_key_string
			.parse()
			.context("failed to parse the raw private key as a PrivateKeySigner")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let signer = EthereumWallet::new(private_key_signer);

		// build the core client
		let core_client = CoreConfig::build(&self.core).await?;

		// build the light node client
		let mut light_node: EthereumClient<FileDB> = EthereumClientBuilder::new()
			.network(self.light_node_config.light_node_network)
			.data_dir(self.light_node_config.light_node_data_dir.clone().into())
			.consensus_rpc(&self.light_node_config.consensus_rpc_url)
			.execution_rpc(&self.core.view_config.rpc_url)
			.load_external_fallback()
			.build()
			.map_err(|e| McrClientError::Internal(e.into()))?;

		// Wait for the light node to sync
		light_node
			.start()
			.await
			.map_err(|e| anyhow::anyhow!("failed to start the light node client: {}", e))
			.map_err(|e| McrClientError::Internal(e.into()))?;
		light_node.wait_synced().await;

		Ok(Client {
			signer,
			core_client,
			eth_client: Arc::new(light_node),
			polling_parameters: PollingParameters::default(),
			block_finality: self.light_node_config.finality.clone().into(),
		})
	}
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct ViewConfig {
	/// The core configuration for the view.
	#[clap(flatten)]
	pub core: CoreViewConfig,

	/// The light node configuration for the view.
	#[clap(flatten)]
	pub light_node_config: LightNodeConfig,
}

impl McrViewOperations for ViewConfig {
	type Config = Config;

	fn try_into_config(self) -> Result<Self::Config, McrClientError> {
		Ok(Config { core: self.core.try_into_config()?, light_node_config: self.light_node_config })
	}
}
