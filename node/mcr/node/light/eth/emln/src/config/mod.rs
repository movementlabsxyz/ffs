use crate::node::Node;
use clap::Parser;
use mcr_protocol_client_eth_emln_core::config::Config as ClientConfig;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
#[group(skip)]
pub struct Config {
	#[clap(flatten)]
	pub client: ClientConfig,
	/// The address to listen on
	#[clap(long, default_value = "0.0.0.0:44513")]
	pub address: String,
}

impl Config {
	pub async fn build(&self) -> Result<Node, anyhow::Error> {
		let client = self.client.build().await?;
		let node = Node { client, address: self.address.clone() };
		Ok(node)
	}
}
