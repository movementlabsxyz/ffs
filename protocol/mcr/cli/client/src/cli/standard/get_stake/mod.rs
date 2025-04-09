#[macro_export]
macro_rules! mcr_get_stake {
	($view_config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{
			McrClientOperations, McrConfigOperations, McrViewOperations,
		};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get the current epoch stake for an attester
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetStake {
			#[clap(flatten)]
			view_config: $view_config,

			/// The custodian address
			#[clap(long)]
			custodian: String,

			/// The attester address
			#[clap(long)]
			attester: String,
		}

		impl GetStake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.view_config.clone().try_into_config()?;
				let client = config.build().await?;
				let stake = client.get_stake(self.custodian.clone(), self.attester.clone()).await?;
				println!("Current stake for {}: {}", self.attester, stake);
				Ok(())
			}
		}

		impl or_file::GetStake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}
