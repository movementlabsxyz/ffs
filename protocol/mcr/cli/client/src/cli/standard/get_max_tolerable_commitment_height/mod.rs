use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};

pub struct GetMaxTolerableCommitmentHeightHelper<V: McrViewOperations> {
	view_config: V,
}

impl<V: McrViewOperations + Clone> GetMaxTolerableCommitmentHeightHelper<V> {
	pub fn new(view_config: V) -> Self {
		Self { view_config }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.view_config.clone().try_into_config()?;
		let client = config.build().await?;

		let height = client.get_max_tolerable_commitment_height().await?;
		println!("Max tolerable commitment height: {}", height);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_get_max_tolerable_commitment_height {
	($view_config:ty) => {
		use crate::cli::standard::get_max_tolerable_commitment_height::GetMaxTolerableCommitmentHeightHelper;
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get max tolerable commitment height
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetMaxTolerableCommitmentHeight {
			/// The view config to use (this is a view method after all).
			#[orfile(config)]
			#[clap(flatten)]
			pub view_config: $view_config,
		}

		impl GetMaxTolerableCommitmentHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetMaxTolerableCommitmentHeightHelper::new(
					self.view_config.clone(),
				);
				helper.execute().await
			}
		}

		impl or_file::GetMaxTolerableCommitmentHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}
