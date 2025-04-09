#[macro_export]
macro_rules! mcr_get_commitment {
	($view_config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{
			McrClientOperations, McrConfigOperations, McrViewOperations,
		};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get the commitment for a given height and attester
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetCommitment {
			/// The view config to use (this is a view method after all).
			#[orfile(config)]
			#[clap(flatten)]
			pub view_config: $view_config,

			/// Block height to check commitment for
			#[clap(long)]
			height: u64,

			/// Attester address to check commitment for
			#[clap(long, required = true)]
			attester: String,
		}

		impl GetCommitment {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let view_config = self.view_config.clone();
				let config = view_config.try_into_config()?;
				let client = config.build().await?;

				println!(
					"Checking commitment for attester {} at height {}",
					self.attester, self.height
				);

				let commitment = client
					.get_validator_commitment_at_height(self.height, self.attester.clone())
					.await?;

				match commitment {
					None => println!("No commitment found"),
					Some(commitment) => {
						println!("Found commitment:");
						println!("{}", commitment);
					}
				}

				Ok(())
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
