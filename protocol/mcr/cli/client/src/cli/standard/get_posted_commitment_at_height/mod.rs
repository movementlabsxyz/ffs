use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GetPostedCommitmentAtHeightArgs {
	/// The height to get the commitment for
	#[clap(long)]
	height: u64,
}

pub struct GetPostedCommitmentAtHeightHelper<C: McrConfigOperations> {
	config: C,
	args: GetPostedCommitmentAtHeightArgs,
}

impl<C: McrConfigOperations + Clone> GetPostedCommitmentAtHeightHelper<C> {
	pub fn new(config: C, args: GetPostedCommitmentAtHeightArgs) -> Self {
		Self { config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.config.clone();
		let client = config.build().await?;

		match client.get_posted_commitment_at_height(self.args.height).await? {
			Some(commitment) => {
				println!("Posted commitment at height {}: {:?}", self.args.height, commitment)
			}
			None => println!("No posted commitment found at height {}", self.args.height),
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_get_posted_commitment_at_height {
	($config:ty) => {
		use crate::cli::standard::get_posted_commitment_at_height::{
			GetPostedCommitmentAtHeightArgs, GetPostedCommitmentAtHeightHelper,
		};
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get posted commitment at a specific height
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetPostedCommitmentAtHeight {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $config,
			/// The arguments for getting the posted commitment
			#[clap(flatten)]
			args: GetPostedCommitmentAtHeightArgs,
		}

		impl GetPostedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetPostedCommitmentAtHeightHelper::new(
					self.view_config.clone(),
					self.args.clone(),
				);
				helper.execute().await
			}
		}

		impl or_file::GetPostedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}
