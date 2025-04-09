#[macro_export]
macro_rules! mcr_unstake {
	($config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, U256};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Unstake tokens from the MCR domain
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct Unstake {
			#[clap(flatten)]
			config: $config,

			/// Amount to unstake
			#[clap(long)]
			amount: U256,
		}

		impl Unstake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let client = self.config.clone().build().await?;
				client.unstake(self.amount).await?;
				println!("Successfully unstaked {} tokens", self.amount);
				Ok(())
			}
		}

		impl or_file::Unstake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}
