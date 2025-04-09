// ! This macro is used to generate the get_accepted_commitment_at_height command.
// ! This is the standard implementation. However, we avoid use generics for two reasons (1) difficulties with [clap] and [serde] when using generics and (2) requiring the developer to flatten the command implementation.

#[macro_export]
macro_rules! mcr_get_accepted_commitment_at_height {
	($view_config:ty) => {
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
			#[clap(long)]
			height: u64,
		}

		impl GetAcceptedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let view_config = self.view_config.clone();
				let config = view_config.try_into_config()?;
				let client = config.build().await?;

				match client.get_accepted_commitment_at_height(self.height).await? {
					Some(commitment) => {
						println!("Commitment at height {}: {:?}", self.height, commitment)
					}
					None => println!("No commitment found at height {}", self.height),
				}

				Ok(())
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
