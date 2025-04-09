#[macro_export]
macro_rules! mcr_stake {
	($config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, U256};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Stake tokens for the MCR domain
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct Stake {
			/// Path to the configuration file
			#[orfile(config)]
			#[clap(flatten)]
			pub config: $config,

			/// Amount to stake
			#[clap(long)]
			amount: U256,
		}

		impl Stake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let client = self.config.build().await?;
				client.stake(self.amount).await?;
				println!("Successfully staked {} tokens", self.amount);
				Ok(())
			}
		}

		impl or_file::Stake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}
