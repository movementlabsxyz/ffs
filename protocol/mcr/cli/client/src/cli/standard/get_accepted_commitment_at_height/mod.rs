// ! This macro is used to generate the get_accepted_commitment_at_height command.
// ! This is the standard implementation. However, we avoid use generics for two reasons (1) difficulties with [clap] and [serde] when using generics and (2) requiring the developer to flatten the command implementation.

use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GetAcceptedCommitmentAtHeightArgs {
	/// The height to get the commitment for
	#[clap(long)]
	height: u64,
}

pub struct GetAcceptedCommitmentAtHeightHelper<V: McrViewOperations> {
	view_config: V,
	args: GetAcceptedCommitmentAtHeightArgs,
}

impl<V: McrViewOperations + Clone> GetAcceptedCommitmentAtHeightHelper<V> {
	pub fn new(view_config: V, args: GetAcceptedCommitmentAtHeightArgs) -> Self {
		Self { view_config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.view_config.clone().try_into_config()?;
		let client = config.build().await?;

		match client.get_accepted_commitment_at_height(self.args.height).await? {
			Some(commitment) => {
				println!("Commitment at height {}: {:?}", self.args.height, commitment);
			}
			None => println!("No commitment found at height {}", self.args.height),
		}

		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_get_accepted_commitment_at_height {
	($view_config:ty) => {
		use crate::cli::standard::get_accepted_commitment_at_height::{
			GetAcceptedCommitmentAtHeightArgs, GetAcceptedCommitmentAtHeightHelper,
		};
		use clap::Parser;
		use mcr_protocol_client_core_util::{
			McrClientOperations, McrConfigOperations, McrViewOperations,
		};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Gets the accepted commitment at a given height.
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetAcceptedCommitmentAtHeight {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $view_config,
			/// The height to get the commitment for
			#[clap(flatten)]
			args: GetAcceptedCommitmentAtHeightArgs,
		}

		impl GetAcceptedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetAcceptedCommitmentAtHeightHelper::new(
					self.view_config.clone(),
					self.args.clone(),
				);
				helper.execute().await
			}
		}

		impl or_file::GetAcceptedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}
