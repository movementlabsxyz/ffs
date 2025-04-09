use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GetCommitmentArgs {
	/// The height to get the commitment for
	#[clap(long)]
	height: u64,
	/// The attester address
	#[clap(long)]
	attester: String,
}

pub struct GetCommitmentHelper<V: McrViewOperations> {
	view_config: V,
	args: GetCommitmentArgs,
}

impl<V: McrViewOperations + Clone> GetCommitmentHelper<V> {
	pub fn new(view_config: V, args: GetCommitmentArgs) -> Self {
		Self { view_config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.view_config.clone().try_into_config()?;
		let client = config.build().await?;

		match client
			.get_validator_commitment_at_height(self.args.height, self.args.attester.clone())
			.await?
		{
			Some(commitment) => println!(
				"Commitment at height {} for {}: {:?}",
				self.args.height, self.args.attester, commitment
			),
			None => println!(
				"No commitment found at height {} for {}",
				self.args.height, self.args.attester
			),
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_get_commitment {
	($view_config:ty) => {
		use crate::cli::standard::get_commitment::{GetCommitmentArgs, GetCommitmentHelper};
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrViewOperations};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get a commitment for a given height and attester
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetCommitment {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $view_config,
			/// The arguments for getting the commitment
			#[clap(flatten)]
			args: GetCommitmentArgs,
		}

		impl GetCommitment {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetCommitmentHelper::new(self.view_config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::GetCommitment {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}
