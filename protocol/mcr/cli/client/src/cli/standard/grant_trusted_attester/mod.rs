#[macro_export]
macro_rules! mcr_grant_trusted_attester {
	($config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Grant the TRUSTED_ATTESTER role to an address
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GrantTrustedAttester {
			#[clap(flatten)]
			config: $config,

			/// The address to grant TRUSTED_ATTESTER role to
			#[clap(long)]
			attester: String,
		}

		impl GrantTrustedAttester {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.config.clone();
				let client = config.build().await?;

				client.grant_trusted_attester(self.attester.clone()).await?;

				Ok(())
			}
		}

		impl or_file::GrantTrustedAttester {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}
