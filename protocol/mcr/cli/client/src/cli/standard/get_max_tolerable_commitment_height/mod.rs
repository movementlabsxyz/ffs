use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GetMaxTolerableCommitmentHeightArgs {
	// No arguments needed for this command
}

pub struct GetMaxTolerableCommitmentHeightHelper<V: McrViewOperations> {
	view_config: V,
	args: GetMaxTolerableCommitmentHeightArgs,
}

impl<V: McrViewOperations + Clone> GetMaxTolerableCommitmentHeightHelper<V> {
	pub fn new(view_config: V, args: GetMaxTolerableCommitmentHeightArgs) -> Self {
		Self { view_config, args }
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
		use crate::cli::standard::get_max_tolerable_commitment_height::{
			GetMaxTolerableCommitmentHeightArgs, GetMaxTolerableCommitmentHeightHelper,
		};
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrViewOperations};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get max tolerable commitment height
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetMaxTolerableCommitmentHeight {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $view_config,
			/// The arguments for getting the max tolerable height
			#[clap(flatten)]
			args: GetMaxTolerableCommitmentHeightArgs,
		}

		impl GetMaxTolerableCommitmentHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetMaxTolerableCommitmentHeightHelper::new(
					self.view_config.clone(),
					self.args.clone(),
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
