use crate::client::Client;
use clap::Parser;
use mcr_light_node_proto::mcr_light_node_service_client::McrLightNodeServiceClient;
use mcr_protocol_client_core_util::McrClientError;
use mcr_protocol_client_core_util::{McrConfigOperations, McrViewOperations};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {
	/// The gRPC endpoint URL for the light node service
	#[clap(long)]
	pub endpoint: String,

	/// The timeout for gRPC requests in milliseconds
	#[clap(long, default_value = "5000")]
	pub timeout_ms: u64,
}

impl McrConfigOperations for Config {
	type Client = Client;

	/// Builds a new client from the configuration
	async fn build(&self) -> Result<Self::Client, McrClientError> {
		let mut endpoint = tonic::transport::Endpoint::from_shared(self.endpoint.clone())
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		// Set timeout
		endpoint = endpoint.timeout(std::time::Duration::from_millis(self.timeout_ms));

		// Configure TLS if using https
		if self.endpoint.starts_with("https://") {
			let tls_config = tonic::transport::ClientTlsConfig::new().with_enabled_roots();
			endpoint = endpoint
				.tls_config(tls_config)
				.map_err(|e| McrClientError::Internal(Box::new(e)))?;

			// Set HTTP/2 keep-alive interval
			endpoint = endpoint.http2_keep_alive_interval(Duration::from_secs(10));
		}

		// Connect to the server
		Ok(Client {
			client: McrLightNodeServiceClient::connect(endpoint)
				.await
				.map_err(|e| McrClientError::Internal(Box::new(e)))?,
		})
	}
}

impl McrViewOperations for Config {
	type Config = Config;

	fn try_into_config(self) -> Result<Self::Config, McrClientError> {
		Ok(self)
	}
}
