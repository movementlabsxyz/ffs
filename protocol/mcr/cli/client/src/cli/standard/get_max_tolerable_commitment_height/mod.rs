#[macro_export]
macro_rules! mcr_get_max_tolerable_commitment_height {
	($view_config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{
			McrClientOperations, McrConfigOperations, McrViewOperations,
		};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get the max tolerable commitment height
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetMaxTolerableCommitmentHeight {
			#[clap(flatten)]
			view_config: $view_config,
		}

		impl GetMaxTolerableCommitmentHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.view_config.clone().try_into_config()?;
				let client = config.build().await?;

				let height = client.get_max_tolerable_commitment_height().await?;
				println!("Max tolerable block height: {}", height);

				Ok(())
			}
		}

		impl or_file::GetMaxTolerableCommitmentHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}
